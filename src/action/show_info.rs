use crate::{util::u8_to_string, wx_util::WeChatInfo};

pub fn show_user_info<FPI>(wechat_info: &WeChatInfo, print_info: FPI) -> anyhow::Result<()> where FPI: Fn(String) -> () {
    print_info(format!("微信版本：\n\t{}", wechat_info.version));
    print_info(format!("用户昵称：\n\t{}", wechat_info.nick_name));
    print_info(format!("用户名：\n\t{}", wechat_info.account));
    print_info(format!("手机号：\n\t{}", wechat_info.mobile));
    print_info(format!(
        "key(hex):\n\t{}",
        u8_to_string(&wechat_info.key, &"hex".to_owned())?
    ));
    print_info(format!(
        "key(base64):\n\t{}",
        u8_to_string(&wechat_info.key, &"base64".to_owned())?
    ));
    Ok(())
}
