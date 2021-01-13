# Basic Timer

## About

This project is a basic implementation of a timer in rust made to be lightweight and accurate.

## Usage

Initialize the timer with

`let timer = Timer::new();`

Then to get the time

`let time = timer.get_time(duration_type: DurationType);`
This returns a u128

duration_type can be one of the following:

- `DurationType::Seconds`
- `DurationType::Milliseconds`
- `DurationType::Microseconds`
- `DurationType::Nanoseconds`

## Example

```rust
use basic_timer::{Timer, DurationType};

fn main() {
	let timer = Timer::new();
	std::thread::sleep(std::time::Duration::new(5, 0));
	let time = timer.get_time(DurationType::Milliseconds);
}
```
