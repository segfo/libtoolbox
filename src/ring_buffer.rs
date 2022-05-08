//! リングバッファの実装です。
//! リングバッファが満タンの状態で新しいデータが追加されると、古いデータから順に削除されます。

pub struct RingBuffer<T> {
    buf: Vec<T>,
    write: usize,    // 次の書き込み位置
    read: usize,     // 次の読み取り位置
    buf_size: usize, // 現在のリングバッファに書き込まれている有効なデータ数
    // 上書きされたデータは無効なものとして扱う=buf_capacityを超えることはない
    buf_capacity: usize, // 実際のリングバッファのキャパ
}
impl<T> RingBuffer<T>
where
    T: Clone + PartialEq + Copy,
{
    /// リングバッファを初期化します。
    pub fn new(buffer_capacity: usize, init_data: T) -> Self {
        RingBuffer {
            buf: vec![init_data; buffer_capacity],
            write: 0,
            read: 0,
            buf_size: 0,
            buf_capacity: buffer_capacity,
        }
    }
    ///
    /// リングバッファへデータを格納します。
    /// リングバッファの容量を超える場合は、古いデータから順に削除されます。
    /// ```
    /// use toolbox::ring_buffer::RingBuffer;
    /// let mut buf = RingBuffer::new(3,0);
    /// // 最大で3個までデータを管理するように設定します。
    /// //（バッファは初期化されなければならないため、0で初期化すると指定しています。）
    /// buf.enqueue(10); // 読み取りはこのデータから始まります。
    /// buf.enqueue(20);
    /// assert_eq!(buf.dequeue(),Some(10));
    /// ```
    ///
    /// ```
    /// use toolbox::ring_buffer::RingBuffer;
    /// let mut buf = RingBuffer::new(3,0);
    /// buf.enqueue(10); // <-- 最後のenqueueにて40を挿入することで、リングバッファは最古の情報（この行で登録された情報）を捨てます
    /// buf.enqueue(20); // <-- 読み取りはこのデータから始まります
    /// buf.enqueue(30);
    /// buf.enqueue(40); // <-- 容量以上のデータをバッファに入れるところ
    /// assert_eq!(buf.dequeue(),Some(20));
    /// ```
    pub fn enqueue(&mut self, data: T) {
        if self.buf_size < self.buf_capacity {
            self.buf_size += 1;
        } else {
            self.read %= self.buf_capacity;
            self.read += 1;
        }
        self.write %= self.buf_capacity;
        self.buf[self.write] = data;
        self.write += 1;
    }
    /// リングバッファからデータを取り出します。
    /// 取り出せない場合にはNoneが返ります。
    /// ```
    /// use toolbox::ring_buffer::RingBuffer;
    /// let mut buf = RingBuffer::new(3,0);
    /// buf.enqueue(10);
    /// buf.enqueue(20);
    /// assert_eq!(buf.dequeue(),Some(10));
    /// assert_eq!(buf.dequeue(),Some(20));
    /// assert_eq!(buf.dequeue(),None); // これ以上データが存在しないためNoneが返却されます。
    /// ```
    pub fn dequeue(&mut self) -> Option<T> {
        if self.buf_size > 0 {
            self.buf_size -= 1;
        } else {
            return None;
        }
        self.read %= self.buf_capacity;
        let data = self.buf[self.read];
        self.read += 1;
        Some(data)
    }
    /// リングバッファからデータを取り出します。
    /// 取り出せない場合にはNoneが返ります。
    /// ```
    /// use toolbox::ring_buffer::RingBuffer;
    /// let mut buf = RingBuffer::new(3,0);
    /// buf.enqueue(10);
    /// buf.enqueue(20);
    /// assert_eq!(buf.len(),2); // キューに2個データが入っているので2が返却されます。
    /// ```
    pub fn len(&self) -> usize {
        self.buf_size
    }

    /// リングバッファに `data` が存在するか確認します。
    /// 存在した場合には、trueが返却されます。
    /// ```
    /// use toolbox::ring_buffer::RingBuffer;
    /// let mut buf = RingBuffer::new(3,0);
    /// buf.enqueue(10);
    /// buf.enqueue(20);
    /// buf.enqueue(30);
    /// buf.enqueue(40);
    /// assert_eq!(buf.contains(30),true); // リングバッファに30が含まれているためtrueが返却されます。
    /// ```
    /// ```
    /// use toolbox::ring_buffer::RingBuffer;
    /// let mut buf = RingBuffer::new(3,0);
    /// buf.enqueue(10);
    /// buf.enqueue(20);
    /// assert_eq!(buf.contains(30),false); // 30はリングバッファに含まれていないためfalseが返却されます。
    /// ```

    pub fn contains(&self, data: T) -> bool {
        self.buf.contains(&data)
    }
}
