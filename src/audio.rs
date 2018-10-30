use std::io::{self, Write};


struct CrasStream {
    buffer_size: usize,
    buffer_a: Vec<u8>,
    buffer_b: Vec<u8>,
    which_buffer: bool,
}

impl CrasStream {
    // TODO(dgreid) support other sample sizes.
    pub fn new(buffer_size: usize, channel_count: usize) -> Self {
        CrasStream {
            buffer_size,
            buffer_a: vec![0; 2 * buffer_size * channel_count],
            buffer_b: vec![0; 2 * buffer_size * channel_count],
            which_buffer: false,
        }
    }

    pub fn next_playback_buffer<'a>(&'a mut self) -> PlaybackBuffer<'a> {
        PlaybackBuffer::new(self)
    }

    fn buffer_complete(&mut self) {
        self.which_buffer = !self.which_buffer;
        // TODO - update write pointer.
    }
}

struct PlaybackBuffer<'a> {
    stream: &'a mut CrasStream,
    offset: usize, // Write offset in frames.
}

impl<'a> PlaybackBuffer<'a> {
    pub fn new(stream: &'a mut CrasStream) -> Self {
        PlaybackBuffer {
            stream,
            offset: 0,
        }
    }
}

impl<'a> Write for PlaybackBuffer<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let written = if self.stream.which_buffer {
            (&mut self.stream.buffer_a[self.offset..]).write(buf)?
        } else {
            (&mut self.stream.buffer_b[self.offset..]).write(buf)?
        };
        self.offset += written;
        Ok(written/4)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> Drop for PlaybackBuffer<'a> {
    fn drop(&mut self) {
        self.stream.buffer_complete();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sixteen_bit_stereo() {
        let mut stream = CrasStream::new(480, 2);
        let mut stream_buffer = stream.next_playback_buffer();
        let pb_buf = [0xa5a5u16; 480];
        assert_eq!(stream_buffer.write(&pb_buf).unwrap(), 480);
    }
}
