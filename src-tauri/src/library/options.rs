/*      My old options array. Kind of heavily modified.

        2023.02.12       Sven Ponelat

*/

use crate::library::functions::*;
use indexmap::*;
use local_timestamps::*;
use regex::Regex;
use std::fs::{self, *};
use std::io::prelude::*;
use anyhow::{Context, Result};


pub const OPTIONS_FILENAME: &str = "./options.txt";
pub const BODY_FILENAME: &str = "../src/body.css";

#[derive(Clone, Debug)]
pub struct SettingsText {
    pub map: IndexMap<String, String>,
}

impl SettingsText {
    pub fn bring_in_options() -> Result<SettingsText, String> {
        match SettingsText::load(OPTIONS_FILENAME) {
            Ok(result) => {
                // check if body.css is there
                match fs::metadata(BODY_FILENAME) {
                    Ok(_) => {
                        // do nothing
                    }

                    Err(_) => {
                        let font_size = result.get_integer("fontSize").unwrap();
                        match make_body_css(BODY_FILENAME, font_size) {
                            Ok(_) => {}
                            Err(e) => return Err(e),
                        }
                    }
                }

                return Ok(result);
            }
            Err(e) => return Err(e),
        }
    }

    // Make a default struct in case it is needed
    pub fn default() -> SettingsText {
        let mut map = IndexMap::new();
        SettingsText::init_map(&mut map);
        SettingsText { map: map }
    }

    // Gets the key from itsself and then parses the string to get the i32
    pub fn get_bool(&self, key: &str) -> Result<bool, String> {
        let result = self.map.get(key);
        match result {
            Some(word) => {
                let first_letter = &(word[0..1].to_owned())[..];
                match first_letter {
                    // if any of below then true
                    "1" | "t" | "T" => return Ok(true),
                    // otherwise false
                    _ => return Ok(false),
                }
            }
            None => {
                let e = format!("Could not find the key '{}' in options file.", key);
                return Err(e);
            }
        }
    }

    // Gets the key from itsself and then parses the string to get the usize
    pub fn get_integer(&self, key: &str) -> Result<usize, String> {
        let temp = self.map.get(key);
        if temp.is_some() {
            let value = temp.unwrap().parse::<usize>();
            if value.is_ok() {
                return Ok(value.unwrap());
            }
        }
        return Err("Could not parse integer in options".to_string());
    }

    fn init_map(map: &mut IndexMap<String, String>) {
        map.insert("fontSize".to_string(), "16".to_string());
        map.insert("lastSpeciesViewed".to_string(), "0".to_string());
        map.insert("lastSightingViewed".to_string(), "0".to_string());
        map.insert("showResponseTimes".to_string(), "1".to_string());
        map.insert("deadBirdIsSighting".to_string(), "1".to_string());
        // map.insert("myBlack".to_string(), "(0, 0, 0)".to_string());
        // map.insert("myBlue".to_string(), "((7,140,245)".to_string());
        // map.insert("myBlueGreen".to_string(), "(0, 177, 177)".to_string());
        // map.insert("myDarkGray".to_string(), "(70, 70, 70)".to_string());
        // map.insert("myGreen".to_string(), "(0, 177, 0)".to_string());
        // map.insert("myLightBlue".to_string(), "(120, 220, 254)".to_string());
        // map.insert("myLightGray".to_string(), "(220, 220, 220)".to_string());
        // map.insert("myNormalGray".to_string(), "(177, 177, 177)".to_string());
        // map.insert("myOlive".to_string(), "(177, 177, 0)".to_string());
        // map.insert("myPurple".to_string(), "(110, 0, 110)".to_string());
        // map.insert("myRed".to_string(), "(177, 0, 0)".to_string());
    }

    // function to load the options file or create a default one
    pub fn load(file: &str) -> Result<SettingsText, String> {
        let ret;

        // does the file exist
        match fs::metadata(file) {
            Ok(_) => match read_option_file_into(file) {
                Ok(settings) => {
                    ret = settings;
                }
                Err(error) => return Err(error),
            },
            // load default
            Err(_) => {
                ret = SettingsText::default();
            }
        }

        Ok(ret)
    }


    // create a new empty SettingsText
    pub fn new() -> SettingsText {
        SettingsText {
            map: IndexMap::new(),
        }
    }

    // Gets the key from itsself and then parses the string to get the i32
    pub fn set_bool(&mut self, key: &str, boolean: bool) -> Result<(), String> {
        let result = self.map.insert(key.to_string(), boolean.to_string());
        if result.is_some() {
            return Ok(());
        }
        let e = format!(
            "Could not insert value: '{}' into key '{}' in options.",
            boolean.to_string(),
            key
        );
        return Err(e);
    }

    // Gets the key from itsself and then parses the string to get the usize
    pub fn set_integer(&mut self, key: &str, size: usize) -> Result<(), String> {
        let result = self.map.insert(key.to_string(), size.to_string());
        if result.is_some() {
            return Ok(());
        }
        let e = format!("Could not insert new value: {}", size);
        return Err(e);
    }

    // // This functions checks if one can read and write to the directory.
    // // Again for testing puposes I have to input a file with a directory.
    // pub fn file_system_ok(test: &str) -> Result<(), String> {
    //     let path = Path::new(test);

    //     // Lets open a file
    //     let mut file = match OpenOptions::new()
    //                                 .read(true)
    //                                 .write(true)
    //                                 .create(true)
    //                                 .open(path){
    //         Ok(content) => content,
    //         Err(_) => { return Err("Problem opening any file in birding program".to_string()); }
    //     };

    //     // Lets write to a file
    //     match file.write_all("Hello Sven".as_bytes()){
    //         Ok(content) => content,
    //         Err(_) => { return Err("Problem writing any file in birding program".to_string()); }
    //     }

    //     // Lets delete a file
    //     match remove_file(&path){
    //         Ok(_) => (),
    //         Err(_) => { return Err("Problem removing any file in birding program".to_string()); }
    //     }

    //     Ok(())
    // }

    // // implement a new BTreeMap
    // pub fn new(options_json: &str) -> SettingsText {

    //     // Read the json file, if it is there and assign it to Settings
    //     // If it s not there, then make a default one
    //     let result = SettingsText::import(options_json);
    //     if result.is_ok() {
    //         return result.unwrap()
    //     }

    //     else {
    //         SettingsText::default()
    //         // SettingsText::init_map(&mut map);
    //         // SettingsText { map: map }
    //     }
    // }

    // // Reads the settings (options.txt) file into a treemap, returning a result
    // pub fn import(path: &str) -> Result<SettingsText, &str> {
    //     let str_file  = std::fs::read_to_string(path );
    //     let content = match str_file {
    //         Ok(content) => { content },
    //         Err(_) => { return Err("Problem reading to String in Settings"); }
    //     };

    //     let m: SettingsText = match serde_json::from_str(&content){
    //         Ok(map) => map,
    //         Err(_) => { return Err("Problem converting to json in Settings"); }
    //     };

    //     Ok(m)
    // }

    // // Gets the key from itsself and then parses the string to get the i32
    // // if errored; return true
    // pub fn get_date_separator(&self) -> String {
    //     let ch = ".".to_string();
    //     let temp = self.map.get("preferredDateSeparatorSymbol");
    //     if temp.is_some() {
    //         let sdata = temp.unwrap().value.to_string();
    //         return sdata;
    //     }
    //     return ch;
    // }

    // // Get the Box Style from options, if errored then return Thick
    // pub fn get_box_style(&mut self, key: &str)  -> Style {

    //     // if file has no error
    //     let temp = self.map.get(key);
    //     if temp.is_some(){

    //         let value = Style::from_str(&temp.unwrap().value.clone());
    //         if value.is_ok(){
    //             return value.unwrap();
    //         }
    //     }

    //     // if error - default to Thick
    //     return Style::Thick
    // }

    // // Writes the settings to disk in local folder
    // pub fn export( &self,  path: &str) -> Result<(), String> {
    //     let path = Path::new(path);

    //      if remove_file(path).is_err() {
    //          let message = format!("No worries: old options file was not found, a new one will be created.");
    //          feedback(Feedback::Info, message)
    //      }

    //     let serialized = serde_json::to_string_pretty(&self);
    //     let mut file = match OpenOptions::new()
    //                             .read(false)
    //                             .write(true)
    //                             .create(true)
    //                             .open(path)  {

    //         Err(_) => { return Err("Problems opening json file in 'write_settings'".to_string()); }
    //         Ok(file)   => { file }
    //     };

    //     match file.write_all(serialized.unwrap().as_bytes()) {
    //         Err(_) => { return Err("Problems writing json file in 'write_settings'".to_string()); }
    //         Ok(file)   => { file }
    //     }

    //     Ok(())
    // }

    // // Apparently inserting just updates the key if present or adds it, if not present.
    // // on insert , if None is returned then key did not initially exist. Although
    // // this scenario is highly unlikely.
    // pub fn set_value_for_key(&mut self, key: &str, value: String)  -> Result<(), String>{

    //     let res = self.map.get(key);
    //     if res.is_some(){

    //         let mut sdata = res.unwrap().to_owned();
    //         sdata.value = value;
    //         let update = self.map.insert(key.to_string(), sdata);

    //         // This is what we want to happen
    //         if update.is_some(){
    //             return Ok(())
    //         }
    //         // Key did not initially exist
    //         else {
    //             return Err("Key did not exist in options file.".to_string());
    //         }
    //     }
    //     // key did not exist
    //     else {
    //         return Err("Key did not exist in options file.".to_string());
    //     }
    //     // self.map.insert(key.to_string(), value);
    // }

    // // Gets the color defined in the options file, if that is corrupt
    // // it will get the default color
    // pub fn get_color(&mut self, key: &str)  -> termion::color::Rgb {

    //     let temp = self.map.get(key);
    //     if temp.is_none() {
    //         return SettingsText::get_default_color(key)
    //     }

    //     let mut org = temp.unwrap().value.clone();
    //     org.retain(|c| !r#"( )"#.contains(c));
    //     let org = org.split(",");
    //     let vec: Vec<&str> = org.collect();

    //     if vec.len() != 3 {
    //         return SettingsText::get_default_color(key)
    //     }

    //     let r = vec[0].parse::<u8>();
    //     let g = vec[1].parse::<u8>();
    //     let b = vec[2].parse::<u8>();

    //     if r.is_err() || g.is_err() || b.is_err() {
    //         return SettingsText::get_default_color(key)
    //     }

    //     termion::color::Rgb (r.unwrap(), g.unwrap(), b.unwrap())
    // }

    // // Returns the default color, there is an assumption that there won't
    // // be any parsing errors etc here
    // pub fn get_default_color(key: &str)  -> termion::color::Rgb {
    //     let def = SettingsText::default();
    //     let mut str_col =   def.map
    //                                 .get(key)
    //                                 .unwrap()
    //                                 .value
    //                                 .clone();

    //     str_col.retain(|c| !r#"( )"#.contains(c));
    //     let org = str_col.split(",");
    //     let vec: Vec<&str> = org.collect();

    //     let r = vec[0].parse::<u8>().unwrap();
    //     let g = vec[1].parse::<u8>().unwrap();
    //     let b = vec[2].parse::<u8>().unwrap();

    //     return termion::color::Rgb(r, g, b)
    // }

    // // Gets the sdata value bound to the given  key
    // pub fn get_value_from_key(&mut self, key: &str) -> Result<Sdata, String> {
    //     let result = self.map
    //                                 .get(key)
    //                                 .clone();

    //     if result.is_none() {
    //         let message = format!("No data found for this key -> {}", key);
    //         return Err(message)
    //     }
    //     Ok( Sdata { value: result.unwrap().value.to_string(), show: result.unwrap().show })
    // }

    // // A function that looks at the preferred separator and returns the date (unix-timestamp)
    // // as a string
    // pub fn get_date_string(&mut self, time: i64) -> String {
    //     let d = UNIX_EPOCH + Duration::from_secs(time as u64);
    //     let datetime = DateTime::<Utc>::from(d);

    //     let temp= datetime.format("%Y.%m.%d").to_string();
    //     // let sep = self.get_value_from_key("preferredDateSeparatorSymbol").unwrap().value;
    //     // if sep == "."{
    //     //     temp = datetime.format("%Y.%m.%d").to_string();
    //     // } else {
    //     //     temp = datetime.format("%Y-%m-%d").to_string();
    //     // }

    //     temp
    // }

    // // This function returns a string with the current date and time {COMPUTER TIME}
    // pub fn date_time_str(&mut self) -> String {
    //     let temp: String;

    //     let secs = chrono::offset::Local::now().naive_local().timestamp();
    //     let d = UNIX_EPOCH + Duration::from_secs(secs as u64);
    //     let datetime = DateTime::<Utc>::from(d);

    //     // let sep = self.get_value_from_key("preferredDateSeparatorSymbol").unwrap().value;
    //     // if sep == "."{
    //         temp = datetime.format("%Y.%m.%d_%H_%M_%S_").to_string();
    //     // } else {
    //     //     temp = datetime.format("%Y-%m-%d_%H_%M_%S_").to_string();
    //     // }

    //     return temp
    // }
} // end of impl SettingsText

//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ Functions @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// get the current font size in the file
pub fn get_file_fontSize(file: &str) -> Result<usize, String> {
    match fs::read_to_string(file) {
        Ok(text) => {
            let re = Regex::new(r"(\d+)px").unwrap();
            match re.captures(&text) {
                Some(getting) => match getting.get(1) {
                    Some(tx) => match tx.as_str().parse::<usize>() {
                        Ok(size) => {
                            if size >= 10 {
                                return Ok(size);
                            }
                            let message = format!("Font size is less than 10px (too small).");
                            return Err(message.to_string());
                        }
                        Err(_) => {
                            let message = format!("Cannot parse number from the {} file", file);
                            return Err(message.to_string());
                        }
                    },
                    None => {
                        let message = format!("Cannot get the index number from the {} file", file);
                        return Err(message.to_string());
                    }
                },
                None => {
                    let message = format!("Getting no captures from the {} file", file);
                    return Err(message.to_string());
                }
            }
        }

        Err(_) => {
            let message = format!("Could not open, find or read the {} file", file);
            return Err(message);
        }
    }
}

// load the options file and return an integer
pub fn get_options_integer(options_filename: &str, key: &str) -> Result<usize, String> {
    match SettingsText::load(options_filename) {
        Ok(st) => match st.get_integer(key) {
            Ok(num) => return Ok(num),
            //bad key
            Err(e) => return Err(e),
        },
        // bad file
        Err(e) => return Err(e),
    }
}

// create the body css with its defaults, file is the name of the file to be created
pub fn make_body_css(file: &str, font_size: usize) -> Result<(), String> {
    // remove the old options file first
    match fs::metadata(file) {
        Ok(_) => match remove_file(file) {
            Ok(_) => {}
            Err(_) => {
                let message = format!("Could not remove the old {} file", file);
                return Err(message);
            }
        },
        Err(_) => {}
    }

    // write the new one
    let body = format!("html {{ font-size: {}px; }}", font_size);
    match fs::write(file, body) {
        Ok(_) => return Ok(()),

        Err(_) => {
            let message = format!("Failed to write file {}", file);
            return Err(message);
        }
    }
}

// Function to make the header in the options file
pub fn make_header() -> String {
    let mut ret = String::new();
    let now = lts_now();
    let date = lts_to_date_string(now);
    let time = lts_to_time_string(now);

    ret.push_str("#######################\n");
    ret.push_str("#                     #\n");
    ret.push_str("# Generated File      #\n");
    ret.push_str("#                     #\n");
    ret.push_str("# Date: ");
    ret.push_str(&date);
    ret.push_str("    #\n");
    ret.push_str("#                     #\n");
    ret.push_str("# Time: ");
    ret.push_str(&time);
    ret.push_str("      #\n");
    ret.push_str("#                     #\n");
    ret.push_str("#######################\n");
    ret.push_str("\n");
    ret.push_str("\n");

    return ret;
}

// Function to read in lines from options file into btreemap
pub fn read_option_file_into(file: &str) -> Result<SettingsText, String> {
    let res_file = File::open(file);
    let mut ret = SettingsText::new();
    let mut line_counter = 0;

    match res_file {
        Ok(_) => {
            // Stores the iterator of lines of the file in lines variable.
            let lines = read_lines(file.to_string());
            // Iterate over the lines of the file.
            for line in lines {
                match line {
                    Ok(lll) => {
                        match lll.len() {
                            // do nothing
                            0 => {}
                            _ => {
                                let ch = lll.chars().nth(0).unwrap();
                                match ch {
                                    // do nothing
                                    '#' => {}
                                    _ => {
                                        let split: Vec<&str> = lll.split("\t").collect();
                                        match split.len() {
                                            1 => {
                                                return Err(
                                                    "Only one split on one line in options file"
                                                        .to_string(),
                                                );
                                            }

                                            2 => {
                                                ret.map.insert(
                                                    split[0].to_string(),
                                                    split[1].to_string(),
                                                );
                                                line_counter += 1;
                                            }
                                            _ => {
                                                return Err("Too many splits on one line in the options file".to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        Err(_) => return Err("Failure to open options.txt".to_string()),
    }

    // make sure something was loaded
    if line_counter < 1 {
        return Err("No lines in option file were loaded".to_string());
    }

    return Ok(ret);
}

pub fn save_options_file(st: SettingsText, file: &str) -> Result<(), String> {
    // if old option file exists - delete it - no worries on error
    let path = file;
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.is_file() {
            let result = fs::remove_file(path);
            if result.is_err() {
                return Err("Error in not removing old options file".to_string());
            }
        }
    }

    let mut block = make_header();

    // add the btreemap
    for (key, value) in st.map.iter() {
        block.push_str(key);
        block.push('\t');
        block.push_str(value);
        block.push('\n');
    }

    match fs::File::create(file) {
        Ok(mut file) => match file.write_all(block.as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err("An error occurred while writing the options file".to_string()),
        },
        Err(_) => return Err("An error occurred while creating the options file".to_string()),
    }

    Ok(())
}

/*
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
                                    ALL TESTS ARE RUN:  ONE AT A TIME

    Running concurrent tests in the same directory with reading and writing has unpredictable results
*/
#[warn(unused_assignments)]
#[cfg(test)]
mod tests {

    //     DONT RUN THE TESTS ABOVE THIS LINE
    use super::*;

    #[ignore]
    #[test]
    fn t001_delete_old_options() {
        let source = "./test/source/options_empty.txt";
        let destination = "./test/working/options.txt";
        copy(source, destination).expect("Failed to copy");

        let remove = remove_file(destination);
        assert_eq!(remove.is_ok(), true);
    }

    #[ignore]
    #[test]
    fn t002_load_default() {
        let destination = "./test/working/options.txt";

        // should produce error
        let remove = remove_file(destination);
        assert_eq!(remove.is_err(), true);

        let result = SettingsText::load(destination);
        assert_eq!(result.clone().unwrap().map.len() > 1, true);

        let result = save_options_file(result.clone().unwrap(), destination);
        assert_eq!(result.is_ok(), true);

        let remove = remove_file(destination);
        assert_eq!(remove.is_err(), false);
    }

    #[ignore]
    #[test]
    fn t003_load_existing() {
        let source = "./test/source/options_empty.txt";
        let destination = "./test/working/options.txt";
        copy(source, destination).expect("Failed to copy");

        // empty options file
        let result = SettingsText::load(destination);
        let _remove = remove_file(destination);
        assert_eq!(result.is_err(), true);

        let source = "./test/source/options_splits.txt";
        let destination = "./test/working/options.txt";
        copy(source, destination).expect("Failed to copy");

        // empty options file
        let result = SettingsText::load(destination);
        let _remove = remove_file(destination);
        assert_eq!(result.is_err(), true);

        let source = "./test/source/options_different.txt";
        let destination = "./test/working/options.txt";
        copy(source, destination).expect("Failed to copy");

        if let Ok(st) = SettingsText::load(destination) {
            let num = st.get_integer("fontSize");
            let _remove = remove_file(destination);
            assert_eq!(num.unwrap(), 17);
        }
    }

    #[ignore]
    #[test]
    fn t004_make_body_css() {
        let source = "./test/source/options_different.txt";
        let destination = "./test/working/options.txt";
        copy(source, destination).expect("Failed to copy");

        // what is the default font-size currently
        let result = get_file_fontSize("./anything.css");
        assert_eq!(result.is_err(), true);

        let source = "./test/source/small_body.css";
        let destination = "./test/working/body.css";
        copy(source, destination).expect("Failed to copy");

        let result = get_file_fontSize("./test/working/anything.css");
        let _remove = remove_file(destination);
        assert_eq!(result.is_err(), true);

        let source = "./test/source/body.css";
        let destination = "./test/working/body.css";
        copy(source, destination).expect("Failed to copy");

        let result = get_file_fontSize(destination);
        let _remove = remove_file(destination);
        assert_eq!(result.unwrap(), 19);
    }

    #[ignore]
    #[test]
    fn t005_make_body_css() {
        let source = "./test/source/options_different.txt";
        let destination = "./test/working/options.txt";
        copy(source, destination).expect("Failed to copy");

        let mut st: SettingsText = SettingsText::new();
        if let Ok(temp) = SettingsText::load(destination) {
            st = temp;
            let _remove = remove_file(destination);
        }

        let source = "./test/source/body.css";
        let destination = "./test/working/body.css";
        copy(source, destination).expect("Failed to copy");

        let result = get_file_fontSize(destination);
        let _remove = remove_file(destination);
        let file_size = result.unwrap();
        assert_eq!(file_size, 19);

        // do we need to change font size
        let current_size = st.get_integer("fontSize").unwrap();
        if current_size != file_size {
            let destination = "./test/working/body.css";
            let result = make_body_css(destination, current_size);
            let _remove = remove_file(destination);
            assert_eq!(result.is_err(), false);
        }
    }

    #[ignore]
    #[test]
    fn t006_bringing_in() {
        let source = "./test/source/options_empty.txt";
        let destination = "./test/working/options.txt";
        copy(source, destination).expect("Failed to copy");

        let load = SettingsText::load(destination);
        let _remove = remove_file(destination);
        assert_eq!(load.is_err(), true);

        let source = "./test/source/options_default.txt";
        let destination = "../src/options.txt";
        copy(source, destination).expect("Failed to copy");

        let load = SettingsText::load(destination);
        // let _remove = remove_file(destination);
        assert_eq!(load.is_err(), false);

        let result = SettingsText::bring_in_options();
        assert_eq!(result.is_ok(), true);
    }

    #[ignore]
    #[test]
    fn t007_set_integer() {
        let source = "./test/source/options_default.txt";
        let destination = "./test/working/options.txt";
        copy(source, destination).expect("Failed to copy");

        let load = SettingsText::load(destination);
        let _remove = remove_file(destination);
        assert_eq!(load.is_ok(), true);

        let mut options = load.unwrap();
        let font_size = options.get_integer("fontSize").unwrap();
        assert_eq!(font_size, 16);

        let new_size = font_size + 1;
        let result = options.set_integer("fontSize", new_size);
        if result.is_ok() {
            let _res = save_options_file(options, destination);
            let load = SettingsText::load(destination);
            options = load.unwrap();
            let font_size = options.get_integer("fontSize").unwrap();
            let _remove = remove_file(destination);
            assert_eq!(font_size, 17);
        }
    }

    #[ignore]
    #[test]
    fn t008_get_set_bool() {
        // get non-existent
        let source = "./test/source/options_15.txt";
        let destination = "./test/working/options.txt";
        copy(source, destination).expect("Failed to copy");

        let mut load = SettingsText::load(destination);
        let _remove = remove_file(destination);
        let key = "showResponseTimesAnd".to_string();
        let show = load.unwrap().get_bool(&key);
        assert_eq!(show.is_err(), true);

        // get bool
        let source = "./test/source/options_bad_different.txt";
        let destination = "./test/working/options.txt";
        copy(source, destination).expect("Failed to copy");

        load = SettingsText::load(destination);
        let _remove = remove_file(destination);
        let key = "showResponseTimes".to_string();
        let show = load.unwrap().get_bool(&key);
        assert_eq!(show.is_ok(), true);
        assert_eq!(show.unwrap(), false);

        let source = "./test/source/options_default.txt";
        let destination = "./test/working/options.txt";
        copy(source, destination).expect("Failed to copy");

        load = SettingsText::load(destination);
        let _remove = remove_file(destination);
        let key = "showResponseTimes".to_string();
        let show = load.clone().unwrap().get_bool(&key);
        assert_eq!(show.is_ok(), true);
        assert_eq!(show.unwrap(), true);

        // set bool nonexistent
        let key = "myOwnIndex".to_string();
        let show = load.clone().unwrap().set_bool(&key, true);
        assert_eq!(show.is_ok(), false);

        // set bool
        let key = "showResponseTimes".to_string();
        let mut options = load.unwrap();
        let _show = options.set_bool(&key, false);
        let _result = save_options_file(options, destination);
        load = SettingsText::load(destination);

        let show = load.clone().unwrap().get_bool(&key);
        assert_eq!(show.is_ok(), true);
        assert_eq!(show.unwrap(), false);
    }

    #[ignore]
    #[test]
    fn t009_get_options_integer() {
        // get non-existent
        let destination = "./test/working/none.txt";
        let num = get_options_integer(destination, "fontSize").unwrap();
        assert_eq!(num, 16);
        
        let source = "./test/source/options_bad_different.txt";
        let destination = "./test/working/options.txt";
        copy(source, destination).expect("Failed to copy");
        
        //key does not have right type
        let num = get_options_integer(destination, "lastSightingViewed");
        assert_eq!(num.is_err(), true);
        
        let source = "./test/source/options_15.txt";
        let destination = "./test/working/options.txt";
        copy(source, destination).expect("Failed to copy");
        
        let num = get_options_integer(destination, "fontSize").unwrap();
        assert_eq!(num, 15);
        
    }
} //end of tests
