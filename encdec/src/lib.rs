pub mod encryption {
    use std::{
        fs::{self, OpenOptions, File},
        io::{self, Write},
        path::Path,
    };
    use rand::Rng;

    const DECRYPT_FOLDER_FILE: &str = ".decrypt";

//////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////CODE BELOW THIS LINE///////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn byte_shift(text: Vec<u8>, shift_by: u8, backwards: bool) -> Vec<u8> {
        let mut new_content: Vec<u8> = vec![];
        /*
        FOR byte_shift EACH ELEMENT OF TEXT IS TO BE READ INDIVIDUALLY AND DEPENDING ON
        THE BOOLEAN backwards THE BYTES WILL EITHER BE INCREASED BY THE VARIABLE shift_by OR BE 
        DECREASED THE SAME AMOUNT
        */
        // ENTER CODE HERE


        

        


//////////////////////////////////////////////////////////////////////////////////////////////////
        new_content
    }

    pub fn process_file(file_path: &Path, decrypting: bool) -> io::Result<()> {
        /*
        THIS FUNCTION process_file WILL TAKE THE PATH OF FILES TO BE ENCRYPTED

        FOLLOWING WHAT WAS EXPLAINED IN THE PPT, THE FILE WILL BE OPENED, READ, WRITTEN IN AND CLOSED
        TO ENCRYPT THE CONTENT THAT WAS READ, THE FUNCTION byte_shift SHOULD BE MADE USE OF

        */
        
        // ENTER CODE HERE







        
//////////////////////////////////////////////////////////////////////////////////////////////////
        Ok(())
    }

    pub fn process_directory(directory_path: &Path, decrypting: bool) -> io::Result<()> {
        /*
        THIS FUNCTION TAKES PATH OF DIRECTORY, READS THE ENTIRE DIRECTORY, ENTRY BY ENTRY AND 
        IF THE ENTRY IS A FILE, IT WILL BE SEND TO THE FUNCITON process_file TO BE ENCRYPTED
        */
        // ENTER CODE HERE






//////////////////////////////////////////////////////////////////////////////////////////////////
        if !decrypting {
            create_decrypt_file(directory_path)?;
        } else {
            remove_decrypt_file(directory_path)?;
        }
    
        Ok(())
    }

    pub fn verify_code(decrypting: bool) -> bool {
        /*
        THIS FUNCTION verify_code() IS A FUNCTION THAT CONFIRMS THE DECRYPTION PROCESS.
        WHEN THIS FUNCTION RETURNS true DECRYPTION PROCESS WILL CONTINUE IN main.rs ELSE THE PROGRAM STOPS

        A RANDOM NUMBER IS TO BE GENERATED AND PRINTED, IF THE USER SUCCESSFULLY ENTER THE SAME CODE
        TRUE IS RETURNED, ELSE FALSE IS RETURNED.

        THE VARIABLE user_input IS TO BE DEFINED AND INPUT WILL BE TAKEN INTO IT FOR CHECKING,
        THE VARIABLE random_code IS TO BE GENERATED AND PRINTED FOR THE USER

        MAKE SURE TO HAVE BOTH VARIABLES HAVE SAME DATA TYPES 
        */

        if decrypting {
            // ENTER CODE HERE








//////////////////////////////////////////////////////////////////////////////////////////////////
            match user_input.trim().parse::<u32>() {
                Ok(code) if code == random_code => true,
                _ => false,
            }
        } else {
            true // No verification required for encryption
        }
    }
    
//////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////CODE ABOVE THIS LINE///////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn create_decrypt_file(directory_path: &Path) -> io::Result<()> {
        let mut decrypt_file = File::create(directory_path.join(DECRYPT_FOLDER_FILE))?;
        decrypt_file.write_all(b"")?;
        Ok(())
    }

    pub fn remove_decrypt_file(directory_path: &Path) -> io::Result<()> {
        let decrypt_file_path = directory_path.join(DECRYPT_FOLDER_FILE);
        fs::remove_file(decrypt_file_path)
    }

    pub fn is_encrypted(directory_path: &Path) -> bool {
        directory_path.join(DECRYPT_FOLDER_FILE).exists()
    }
}