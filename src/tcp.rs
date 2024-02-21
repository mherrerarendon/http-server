use anyhow::bail;
use tokio::{io::AsyncReadExt, time::timeout};

pub async fn read_from_stream_until_null<R>(stream: &mut R) -> anyhow::Result<Vec<u8>>
where
    R: AsyncReadExt + std::marker::Unpin,
{
    let timeout_duration = tokio::time::Duration::from_millis(500);
    const BUFF_SIZE: usize = 5;
    let mut request_buff = [0u8; BUFF_SIZE];
    let mut request_bytes: Vec<u8> = Vec::new();
    let mut total_bytes_read = 0;
    loop {
        request_buff.iter_mut().for_each(|b| *b = 0);
        match timeout(timeout_duration, stream.read(&mut request_buff)).await? {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                } else {
                    total_bytes_read += bytes_read;
                    request_bytes.extend_from_slice(&request_buff);
                    if request_buff[BUFF_SIZE - 1] == 0 {
                        break;
                    }
                }
            }
            Err(_) => bail!("Connection timed out"),
        }
    }
    request_bytes.truncate(total_bytes_read);
    Ok(request_bytes)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[tokio::test]
    async fn it_reads_with_all_zeroes() -> anyhow::Result<()> {
        let mut cursor = Cursor::new(vec![0; 15]);
        let r = read_from_stream_until_null(&mut cursor).await?;
        assert_eq!(r.len(), 5);
        Ok(())
    }

    #[tokio::test]
    async fn it_reads_with_all_ones() -> anyhow::Result<()> {
        let mut cursor = Cursor::new(vec![1; 15]);
        let r = read_from_stream_until_null(&mut cursor).await?;
        assert_eq!(r.len(), 15);
        Ok(())
    }

    #[tokio::test]
    async fn it_reads_with_zeroes_at_end() -> anyhow::Result<()> {
        let mut bytes: Vec<u8> = vec![1; 15];
        bytes[9..].iter_mut().for_each(|b| *b = 0);
        let mut cursor = Cursor::new(bytes);
        let r = read_from_stream_until_null(&mut cursor).await?;
        assert_eq!(r.len(), 10);
        Ok(())
    }

    #[tokio::test]
    async fn it_reads_zero_bytes() -> anyhow::Result<()> {
        let mut cursor = Cursor::new(vec![]);
        let r = read_from_stream_until_null(&mut cursor).await?;
        assert_eq!(r.len(), 0);
        Ok(())
    }
}
