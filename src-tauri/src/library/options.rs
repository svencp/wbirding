/*      My old options array. Kind of heavily modified.

        2023.02.12       Sven Ponelat

*/

use crate::library::functions::*;
use local_timestamps::*;
use indexmap::*;
use std::collections::BTreeMap;
use std::fs::{self, *};
use std::io::prelude::*;
use std::path::Path;

pub const OPTIONS_FILENAME: &str = "./options.txt";
// #[rustfmt::skip]
// #[derive(Clone, Debug)]
// pub struct Sdata {
//     pub value: String,
//     pub show: bool,
// }

#[derive(Clone, Debug)]
pub struct SettingsText {
    pub map: IndexMap<String, String>,
}

impl SettingsText {
    // Make a default struct in case it is needed
    pub fn default() -> SettingsText {
        let mut map = IndexMap::new();
        SettingsText::init_map(&mut map);
        SettingsText { map: map }
    }

    pub fn save_options_file(self: SettingsText) -> Result<(), String> {
        // if old option file exists - delete it - no worries on error
        let path = OPTIONS_FILENAME;
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
        for (key, value) in self.map.iter() {
            block.push_str(key);
            block.push('\t');
            block.push_str(value);
            block.push('\n');
        }

        match fs::File::create(OPTIONS_FILENAME) {
            Ok(mut file) => match file.write_all(block.as_bytes()) {
                Ok(_) => (),
                Err(_) => {
                    return Err("An error occurred while writing the options file".to_string())
                }
            },
            Err(_) => return Err("An error occurred while creating the options file".to_string()),
        }

        Ok(())
    }

    // function to load the options file or create a default one
    pub fn load(file: &str) -> Result<SettingsText, String> {
        let mut ret = SettingsText::default();
        let path = file;

        // if options exist then load that one otherwise the default is already loaded
        let exists = fs::metadata(path);
        if exists.is_ok() {
            let result = read_option_file_into(&mut ret);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            return Ok(result.unwrap());
        }

        Ok(ret)
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

    fn init_map(map: &mut IndexMap<String, String>) {
        map.insert("fontSize".to_string(), "16".to_string());
        map.insert("lastSpeciesViewed".to_string(), "0".to_string());
        map.insert("lastSightingViewed".to_string(), "0".to_string());
        map.insert("myBlack".to_string(), "(0, 0, 0)".to_string());
        map.insert("myBlue".to_string(), "((7,140,245)".to_string());
        map.insert("myBlueGreen".to_string(), "(0, 177, 177)".to_string());
        map.insert("myDarkGray".to_string(), "(70, 70, 70)".to_string());
        map.insert("myGreen".to_string(), "(0, 177, 0)".to_string());
        map.insert("myLightBlue".to_string(), "(120, 220, 254)".to_string());
        map.insert("myLightGray".to_string(), "(220, 220, 220)".to_string());
        map.insert("myNormalGray".to_string(), "(177, 177, 177)".to_string());
        map.insert("myOlive".to_string(), "(177, 177, 0)".to_string());
        map.insert("myPurple".to_string(), "(110, 0, 110)".to_string());
        map.insert("myRed".to_string(), "(177, 0, 0)".to_string());
        map.insert("deadBirdIsSighting".to_string(), "true".to_string());
        map.insert("showResponseTimes".to_string(), "true".to_string());
    }

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

    // Gets the key from itsself and then parses the string to get the i32
    // if errored; return true
    pub fn get_bool(&self, key: &str) -> Result<bool, String> {
        let temp = self.map.get(key);
        if temp.is_some() {
            let value = temp.unwrap().parse::<bool>();
            if value.is_ok() {
                return Ok(value.unwrap());
            }
        }
        return Err("Could not parse boolean in options".to_string());
    }

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
pub fn read_option_file_into(st: &mut SettingsText) -> Result<&mut SettingsText, String> {
    let res_file = File::open(OPTIONS_FILENAME);
    match res_file {
        Ok(_) => {
            // Stores the iterator of lines of the file in lines variable.
            let lines = read_lines(OPTIONS_FILENAME.to_string());
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
                                                return Err("Only one split on one line in options file".to_string());
                                            }
                                            
                                            2 => {
                                                    st.map.insert(split[0].to_string(), 
                                                    split[1].to_string());
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

    return Ok(st);
}

// fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
//     // Open the file in read-only mode.
//     let file = File::open(filename).unwrap();
//     // Read the file line by line, and return an iterator of the lines of the file.
//     return io::BufReader::new(file).lines();
// }

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
    use std::fs::*;

    #[ignore]
    #[test]
    fn t001_delete_old_options() {
        let source = "./test/source/empty-options.txt";
        let destination = "./test/working/options.txt";
        copy(source, destination).expect("Failed to copy");
        
        let remove = remove_file(destination);
        assert_eq!(remove.is_err(), false);
    }
    
    #[ignore]
    #[test]
    fn t002_options_file_create_default() {
        let op = SettingsText::default();
        let result = SettingsText::save_options_file(op);
        assert_eq!(result.is_err(), false);
    }
    
    #[ignore]
    #[test]
    fn t003_read_options_save() {
        let source = "./test/source/op_15.txt";
        let destination = "./test/working/options.txt";
        copy(source, destination).expect("Failed to copy");
        
        let mut ret = SettingsText::default();
        let result = read_option_file_into(&mut ret);
        
        let _remove = remove_file(destination);
        assert_eq!(result.unwrap().map.len(), 15);
    }
} //end of tests
