use strip_bom::StripBom;

fn main()
{
 let my_string: String = std::fs::read_to_string("tests/bom-sushi.txt").unwrap();

 // In this time, my_string has the BOM => true 🍣
 println!("{} {}", my_string.starts_with("\u{feff}"), &my_string);

 // Strip BOM
 let my_string: &str = my_string.strip_bom();

 // my_string (slice) has not the BOM => false 🍣
 println!("{} {}", my_string.starts_with("\u{feff}"), &my_string);
}
