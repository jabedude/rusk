pub mod ffi;
mod img;


#[cfg(test)]
mod tests {
    use crate::*;
    use std::ffi::CString;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_ffi_tsk_img_open() {
        let test_file = CString::new("test.vmdk").unwrap();
        let mode = ffi::TSK_IMG_TYPE_ENUM_TSK_IMG_TYPE_DETECT;
        unsafe {
        let img = ffi::tsk_img_open_sing(test_file.as_ptr(), mode, 0);
        println!("Img: {:?}", (*img).itype);
        println!("Img: {:?}", (*img).size);
        assert_eq!((*img).itype, ffi::TSK_IMG_TYPE_ENUM_TSK_IMG_TYPE_VMDK_VMDK);
        }
    }
}
