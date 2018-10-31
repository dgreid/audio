extern crate sample;

use std::io::{self, Write};

use sample::Frame;

trait AudioBufferStream<F: Default + Frame> {
    fn next_playback_buffer<'a>(&'a mut self) -> PlaybackBuffer<'a, F>;
}

struct CrasStream<F: Frame + Default> {
    buffer_size: usize,
    buffer_a: Vec<F>,
    buffer_b: Vec<F>,
    which_buffer: bool,
}

impl<F: Default + Frame> CrasStream<F> {
    // TODO(dgreid) support other sample sizes.
    pub fn new(buffer_size: usize) -> Self {
        CrasStream {
            buffer_size,
            buffer_a: vec![Default::default(); buffer_size],
            buffer_b: vec![Default::default(); buffer_size],
            which_buffer: false,
        }
    }
}

impl<F: Default + Frame> AudioBufferStream<F> for CrasStream<F> {
    fn next_playback_buffer<'a>(&'a mut self) -> PlaybackBuffer<'a, F> {
        PlaybackBuffer::new(&mut self.which_buffer, &mut self.buffer_a) // TODO buffer select
    }
}

struct PlaybackBuffer<'a, F: 'a + Default + Frame> {
    stream: &'a mut bool,
    buffer: &'a mut [F],
    offset: usize, // Write offset in frames.
}

impl<'a, F: Default + Frame> PlaybackBuffer<'a, F> {
    pub fn new(stream: &'a mut bool, buffer: &'a mut [F]) -> Self {
        PlaybackBuffer {
            stream,
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
        *self.stream = !*self.stream;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sixteen_bit_stereo() {
        let mut stream: CrasStream<[u16; 2]> = CrasStream::new(480);
        let mut stream_buffer = stream.next_playback_buffer();
        let pb_buf = [[0xa5a5u16; 2]; 480];
        assert_eq!(stream_buffer.write_samples(&pb_buf).unwrap(), 480);
    }
}
