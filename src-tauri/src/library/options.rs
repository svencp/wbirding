/*      My old options array. Kind of heavily modified.

        2023.02.12       Sven Ponelat
 
*/


use std::{collections::BTreeMap, path::Path, fs::OpenOptions};



pub const OPTIONS_FILENAME: &str = "./options.txt";



#[derive(Clone, Debug)]
pub struct Sdata {
    pub value: String,
    pub show: bool
}

#[derive(Clone, Debug)]
pub struct SettingsText {
   pub map: BTreeMap<String,Sdata>
}


impl   SettingsText {

    // Make a default struct in case it is needed
    pub fn default() -> SettingsText {

        let mut map = BTreeMap::new();
        SettingsText::init_map(&mut map);
        SettingsText { map: map }
    }

    pub fn save_options_file() -> Result<(),String> {

        // if old option file exists - delete it


        Ok(())
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
    
    
    fn init_map(map: &mut BTreeMap<String,Sdata>) {
        map.insert("lastSpeciesViewed".to_string(), Sdata { value: "0".to_string(), show: false });
        map.insert("lastSightingViewed".to_string(), Sdata { value: "0".to_string(), show: false });
        map.insert("myBlack".to_string(), Sdata { value: "(0, 0, 0)".to_string(), show: true });
        map.insert("myBlue".to_string(), Sdata { value: "((7,140,245)".to_string(), show: true });
        map.insert("myBlueGreen".to_string(), Sdata { value: "(0, 177, 177)".to_string(), show: true });            //  5
        map.insert("myDarkGray".to_string(), Sdata { value: "(70, 70, 70)".to_string(), show: true });
        map.insert("myGreen".to_string(), Sdata { value: "(0, 177, 0)".to_string(), show: true });
        map.insert("myLightBlue".to_string(), Sdata { value: "(120, 220, 254)".to_string(), show: true });
        map.insert("myLightGray".to_string(), Sdata { value: "(220, 220, 220)".to_string(), show: true });
        map.insert("myNormalGray".to_string(), Sdata { value: "(177, 177, 177)".to_string(), show: true });         // 10
        map.insert("myOlive".to_string(), Sdata { value: "(177, 177, 0)".to_string(), show: true });
        map.insert("myPurple".to_string(), Sdata { value: "(110, 0, 110)".to_string(), show: true });
        map.insert("myRed".to_string(), Sdata { value: "(177, 0, 0)".to_string(), show: true });
        map.insert("speciesBoxStyle".to_string(), Sdata { value: "Thick".to_string(), show: true });
        map.insert("deadBirdIsSighting".to_string(), Sdata { value: "true".to_string(), show: true });              // 15
        map.insert("showResponseTimes".to_string(), Sdata { value: "true".to_string(), show: true });
        map.insert("showSpeciesIndex".to_string(), Sdata { value: "true".to_string(), show: true });



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


    // Gets the key from itsself and them parses the string to get the i32
    // if errored; return 0
    pub fn get_number(&self, key: &str) -> usize {

        let temp = self.map.get(key);
        if temp.is_some(){
            let value = temp.unwrap().value.parse::<usize>();
            if value.is_ok(){
                return value.unwrap()
            }
        }
        return 0
    }  


    // Gets the key from itsself and then parses the string to get the i32
    // if errored; return true
    pub fn get_bool(&self, key: &str) -> bool {
        
        let temp = self.map.get(key);
        if temp.is_some(){
            let value = temp.unwrap().value.parse::<bool>();
            if value.is_ok(){
                return value.unwrap()
            }
        }
        return true
    }

    
    // Gets the key from itsself and then parses the string to get the i32
    // if errored; return true
    pub fn get_date_separator(&self) -> String {
        let ch = ".".to_string();
        let temp = self.map.get("preferredDateSeparatorSymbol");
        if temp.is_some(){
            let sdata = temp.unwrap().value.to_string();
            return sdata
        }
        return ch
    }


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





























/*
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
                                    ALL TESTS ARE RUN:  ONE AT A TIME   
                                    
    Running concurrent tests in the same directory with reading and writing has unpredictable results                                    
*/
#[warn(unused_assignments)]
#[cfg(test)]
mod tests {                   //     DONT RUN THE TESTS ABOVE THIS LINE
  use super::*;
  use std::{fs::copy};

//   #[ignore]
  #[test]
  fn t001_delete_old_options() {
    let source = "./test/source/empty-options.txt";
    let destination = "./test/working/options.txt";
    copy(source,destination).expect("Failed to copy");


      assert_eq!(true, true);
  }












} //end of tests