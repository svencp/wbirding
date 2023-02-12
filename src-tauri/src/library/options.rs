/*      My old options array. Kind of heavily modified.

        2023.02.12       Sven Ponelat
 
*/

use std::collections::BTreeMap;


#[derive(Clone, Debug)]
pub struct Sdata {
    pub value: String,
    pub show: bool
}

#[derive(Clone, Debug)]
pub struct SettingsText {
   pub map: BTreeMap<String,Sdata>
}































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

  #[ignore]
  #[test]
  fn t001_file_system_ok() {
      let test_file = "./test/test_file1.txt";
      let s1 = SettingsText::file_system_ok(test_file);

      assert_eq!(s1.is_ok(), true);
  }












} //end of tests