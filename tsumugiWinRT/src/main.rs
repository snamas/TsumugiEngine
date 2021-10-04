use tsumugiWinRT::windowsBuild::Windows::Data::Xml::Dom::XmlDocument;
use tsumugiWinRT::windowsBuild::Windows::Win32::Foundation::CloseHandle;
use tsumugiWinRT::windowsBuild::Windows::Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject};
use tsumugiWinRT::windowsBuild::Windows::Win32::UI::WindowsAndMessaging::{MB_OK, MessageBoxA};

fn main() -> windows::Result<()> {
    let mut path = ::std::path::PathBuf::from(
        ::std::env::var("OUT_DIR").expect("No `OUT_DIR` env variable set"),
    );

    println!("{:?}", path);
    let doc = XmlDocument::new()?;

    doc.LoadXml("<html>hello world</html>")?;

    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    assert!(root.InnerText()? == "hello world");

    unsafe {
        let event = CreateEventW(std::ptr::null_mut(), true, false, None);
        SetEvent(event).ok()?;
        WaitForSingleObject(event, 0);
        CloseHandle(event).ok()?;

        MessageBoxA(None, "Text", "Caption", MB_OK);
    }

    Ok(())
}