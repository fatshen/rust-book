fn main() {
    // Drop
    {
        struct HasDrop;

        impl Drop for HasDrop {
            fn drop(&mut self) {
                println!("Dropping");
            }
        }

        let _x = HasDrop;
    }

    // order
    {
        struct Firework {
            strength: i32,
        }

        impl Drop for Firework {
            fn drop(&mut self) {
                println!("strength: {}", self.strength);
            }
        }

        let _a = Firework { strength: 1 };
        let _b = Firework { strength: 100 };
    }
}
