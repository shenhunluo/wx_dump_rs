use crate::{util::u8_to_string, wx_util::WeChatInfo};

pub fn show_user_info(wechat_info: &WeChatInfo) -> anyhow::Result<()> {
    println!("微信版本：\n\t{}", wechat_info.version);
    println!("用户昵称：\n\t{}", wechat_info.nick_name);
    println!("用户名：\n\t{}", wechat_info.account);
    println!("手机号：\n\t{}", wechat_info.mobile);
    println!(
        "key(hex):\n\t{}",
        u8_to_string(&wechat_info.key, &"hex".to_owned())?
    );
    println!(
        "key(base64):\n\t{}",
        u8_to_string(&wechat_info.key, &"base64".to_owned())?
    );
    Ok(())
}
