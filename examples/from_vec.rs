use strip_bom::StripBom;

fn main()
{
 let my_string: Vec<u8> = vec![0xefu8, 0xbb, 0xbf, 0xf0, 0x9f, 0x8d, 0xa3];
 let my_string: String = String::from_utf8(my_string).unwrap();

 // In this time, my_string has the BOM => true 🍣
 println!("{} {}", my_string.starts_with("\u{feff}"), &my_string);

 // Strip BOM
 let my_string: &str = my_string.strip_bom();

 // my_string (slice) has not the BOM => false 🍣
 println!("{} {}", my_string.starts_with("\u{feff}"), &my_string);
}
