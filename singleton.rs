struct Singleton {
    magic: i32,
}

impl Singleton {
    fn new() -> Self {
        Singleton { magic: 0 }
    }

    fn get_data(&self) -> i32 {
        self.magic
    }

    fn set_data(&mut self, data: i32) {
        self.magic = data;
    }
}

static mut INSTANCE: Option<Singleton> = None;

fn get_instance() -> &'static mut Singleton {
  unsafe {
      INSTANCE.get_or_insert_with(Singleton::new)
  }
}

fn main() {
  let instance1 = get_instance();
  let instance2 = get_instance();

  assert_eq!(instance1.get_data(), 0);
  assert_eq!(instance2.get_data(), 0);

  instance1.set_data(42);

  assert_eq!(instance1.get_data(), 42);
  assert_eq!(instance2.get_data(), 42);
}