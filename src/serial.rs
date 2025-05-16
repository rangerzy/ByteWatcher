use memchr::memmem;
use std::io;
use tokio::io::AsyncReadExt;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;
use tokio_serial::{SerialPortBuilderExt, SerialStream};
// use tokio_util::codec::{FramedRead, LinesCodec};
use tokio_util::{
    bytes::BytesMut,
    codec::{Decoder, Encoder},
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct LineCodec;
impl LineCodec {
    pub fn new() -> LineCodec {
        LineCodec {}
    }
}
const DELIMITER: &[u8] = &[0x55, 0xAA];
impl Decoder for LineCodec {
    // type Item = Vec<BytesMut>;
    type Item = BytesMut;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // let newline = src.as_ref().iter().position(|b| *b == b'\n');
        // let frame = src.windows(2).position(|w| w == DELIMITER);
        // let frame = memmem::find(src, DELIMITER);
        // if let Some(index) = frame {
        //     let line = src.split_to(index + 1);
        //     return Ok(Some(line));
        // }
        //
        let frame: Vec<usize> = memmem::find_iter(src, DELIMITER).collect();
        println!("{:?}", frame);
        // println!("index {:?}", frame);
        // for ele in frame {}
        // if frame.len() > 1 {
        //     return Ok(Some(line));
        // }
        Ok(None)
    }

    fn decode_eof(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.decode(buf)? {
            Some(frame) => Ok(Some(frame)),
            None => {
                if buf.is_empty() {
                    Ok(None)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "bytes remaining on stream").into())
                }
            }
        }
    }

    fn framed<T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Sized>(
        self,
        io: T,
    ) -> tokio_util::codec::Framed<T, Self>
    where
        Self: Sized,
    {
        tokio_util::codec::Framed::new(io, self)
    }
}

impl Encoder<String> for LineCodec {
    type Error = io::Error;

    fn encode(&mut self, _item: String, _dst: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub struct Serial {
    shutdown_tx: mpsc::Sender<()>,
    handle: Mutex<Option<JoinHandle<()>>>,
    pub data_rx: mpsc::Receiver<BytesMut>,
}
impl Serial {
    pub fn new(path: &str, baud_rate: u32, data_bits: u8, stop_bits: u8) -> Self {
        let port = tokio_serial::available_ports().unwrap();
        for ele in port {
            println!("{}:{:?}", ele.port_name, ele.port_type);
        }
        let (shutdown_tx, shutdown_rx): (mpsc::Sender<()>, mpsc::Receiver<()>) = mpsc::channel(1);
        let (data_tx, data_rx): (mpsc::Sender<BytesMut>, mpsc::Receiver<BytesMut>) =
            mpsc::channel(1);
        let port = tokio_serial::new(path, baud_rate)
            .data_bits(tokio_serial::DataBits::try_from(data_bits).unwrap())
            .stop_bits(tokio_serial::StopBits::try_from(stop_bits).unwrap())
            .open_native_async()
            .unwrap();
        let handle = tokio::spawn(Self::read(port, shutdown_rx, data_tx));
        Self {
            shutdown_tx: shutdown_tx,
            handle: Mutex::new(Some(handle)),
            data_rx,
        }
    }
    pub async fn read(
        mut port: SerialStream,
        mut shutdown_rx: mpsc::Receiver<()>,
        data_tx: mpsc::Sender<BytesMut>,
    ) {
        // let mut reader = LineCodec.framed(port);
        // let mut reader = FramedRead::new(port, LineCodec::new());
        let mut buf = [0; 1024];

        loop {
            tokio::select! {
                res = port.read(&mut buf) => {
                    match res {
                        Ok(n) => {
                            println!("nnnn");
                            let mut b = BytesMut::new();
                            b.extend_from_slice(&buf[..n]);
                            let _ = data_tx.send(b).await;
                        },
                        Err(e) => eprintln!("Read error: {}", e),
                    }
                }
                _ = shutdown_rx.recv() => {
                    println!("Shutting down worker");
                    break;
                }
            }
        }
    }
    pub fn close(&self) {
        futures::executor::block_on(async {
            if let Some(handle) = self.handle.lock().await.take() {
                let _ = self.shutdown_tx.send(()).await;
                handle.await.unwrap();
            }
        });
    }
}
