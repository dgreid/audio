extern crate sample;

use std::marker::PhantomData;

use sample::{Frame, Signal};

struct CrasStream<'a, F: 'a + Frame + Copy + Clone> {
    buffer_size: usize,
    buffer_a: Vec<F>,
    buffer_b: Vec<F>,
    which_buffer: bool,
    frame_type: PhantomData<&'a F>,
}

impl<'a, F:Frame + Copy + Clone> CrasStream<'a, F> {
    pub fn new(buffer_size: usize) -> Self {
        CrasStream {
            buffer_size,
            buffer_a: Vec::new(),
            buffer_b: Vec::new(),
            which_buffer: false,
            frame_type: PhantomData,
        }
    }

//    pub fn next_buffer(&mut self) -> std::result::Result<impl Signal<Frame=F>, ()> {
 //       Err(())
  //  }

//    pub fn next_buffer(&mut self) -> signal::Take<Signal<Frame=F>> {
 //       self.sine.take(self.buffer_size)
  //  }
  //
    pub fn next_block(&mut self) -> StreamBlock<F> {
        StreamBlock::new(&mut self.buffer_a)
    }

    pub fn next_block_iter(&mut self) -> impl Iterator<Item = &mut F> {
        self.buffer_a.iter_mut()
    }
}

impl<'a, F: Frame> Iterator for CrasStream<'a, F>
{
    type Item = Iterator<Item = F>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_block_iter())
    }
}

struct StreamBlock<'a, F: 'static + Frame + Copy + Clone> {
    buffer: &'a mut [F],
    frame_type: PhantomData<F>,
}

impl<'a, F: Frame + Copy + Clone> StreamBlock<'a, F> {
    pub fn new(buffer: &'a mut [F]) -> Self {
        StreamBlock {
            buffer,
            frame_type: PhantomData,
        }
    }
}

impl<'a, F: Frame + Default> Signal for StreamBlock<'a, F> {
    type Frame = F;

    fn next(&mut self) -> Self::Frame {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sixteen_bit_stereo() {
        let mut stream: CrasStream<[i16; 2]> = CrasStream::new(480);
        let mut signal = stream.next_block().take(3);
        assert_eq!(signal.next(), Some([0,0]));
    }

    #[test]
    fn get_mut_iter() {
        let mut stream: CrasStream<[i16; 2]> = CrasStream::new(480);
        let mut signal = stream.next_block_iter().take(3);
        assert_eq!(signal.next(), Some(&mut [0,0]));
    }
}
