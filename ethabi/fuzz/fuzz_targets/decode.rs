#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (Vec<ethabi::ParamType>, &[u8])| {
	let (types, data) = data;
	let _ = ethabi::decode(&types, data);
});
