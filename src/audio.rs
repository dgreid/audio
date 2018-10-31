extern crate sample;

use std::io;

use sample::Frame;

pub trait PlaybackBufferStream<F: Default + Frame> {
    fn next_playback_buffer<'a>(&'a mut self) -> PlaybackBuffer<'a, F>;
}

pub struct PlaybackBuffer<'a, F: 'a + Default + Frame> {
    done_toggle: &'a mut bool,
    buffer: &'a mut [F],
    offset: usize, // Write offset in frames.
}

impl<'a, F: Default + Frame> PlaybackBuffer<'a, F> {
    pub fn new(done_toggle: &'a mut bool, buffer: &'a mut [F]) -> Self {
        PlaybackBuffer {
            done_toggle,
            buffer,
            offset: 0,
        }
    }

    pub fn write_samples(&mut self, samples: &[F]) -> io::Result<usize> {
        let s = &mut self.buffer[self.offset..];
        let mut i = 0;
        for sample in samples {
            s[i] = *sample;
            i+=1;
        }
        Ok(i)
    }
}

impl<'a, F: Default + Frame> Drop for PlaybackBuffer<'a, F> {
    fn drop(&mut self) {
        *self.done_toggle = !*self.done_toggle;
    }
}

/// Stream that accepts playback samples but drops them.
pub struct DummyStream<F: Frame + Default> {
    buffer: Vec<F>,
    which_buffer: bool,
}

impl<F: Default + Frame> DummyStream<F> {
    pub fn new(buffer_size: usize) -> Self {
        DummyStream {
            buffer: vec![Default::default(); buffer_size],
            which_buffer: false,
        }
    }
}

impl<F: Default + Frame> PlaybackBufferStream<F> for DummyStream<F> {
    fn next_playback_buffer<'a>(&'a mut self) -> PlaybackBuffer<'a, F> {
        PlaybackBuffer::new(&mut self.which_buffer, &mut self.buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sixteen_bit_stereo() {
        let mut stream: DummyStream<[u16; 2]> = DummyStream::new(480);
        let mut stream_buffer = stream.next_playback_buffer();
        let pb_buf = [[0xa5a5u16; 2]; 480];
        assert_eq!(stream_buffer.write_samples(&pb_buf).unwrap(), 480);
    }
}
