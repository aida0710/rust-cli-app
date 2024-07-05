use pcap::{Capture, Device};
use crate::packet_parser;

pub fn packet_capture(capture_device: Device) -> Result<(), Box<dyn std::error::Error>> {
    let mut cap = Capture::from_device(capture_device)?
        .promisc(true) // プロミスキャスモードを有効にする(自分宛以外のパケットもキャプチャする)
        .snaplen(65535) // キャプチャするパケットの最大サイズを指定(65535バイトはEthernetフレームの最大サイズなので、実質無制限)
        .buffer_size(5 * 1024 * 1024) // バッファサイズを5MBに指定
        .immediate_mode(true) // キャプチャを開始するとすぐにパケットを取得する
        .open()?;

    let mut save_file = cap.savefile("capture.pcap")?;
    println!("パケットキャプチャを開始します...");
    let mut count = 0;

    // パケットをキャプチャして表示
    loop {
        match cap.next_packet() {
            Ok(packet) => {
                save_file.write(&packet);

                if let Some((src_ip, dst_ip, protocol)) = packet_parser::parse_packet(&packet.data) {
                    println!("#{count} {src} > {dst} {proto}", count = count, src = src_ip, dst = dst_ip, proto = protocol);
                }
                if count >= 5000 {
                    println!("5000個のパケットをキャプチャしました。終了します。");
                    break;
                };
                count += 1;
            }
            Err(pcap::Error::TimeoutExpired) => continue,
            Err(e) => {
                eprintln!("パケットの取得中にエラーが発生しました: {}", e);
                break;
            }
        }
    }

    save_file.flush()?;
    println!("キャプチャ内容をファイルに保存しました。");

    Ok(())
}