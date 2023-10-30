use std::{
    collections::HashMap,
    fs::{create_dir, read_dir, File},
    io::stdin,
    path::{Path, PathBuf},
};

use anyhow::anyhow;

pub fn get_database(
    wechat_path: &Option<String>,
    save_dir: &String,
    account: &String,
) -> anyhow::Result<()> {
    let mut map = HashMap::new();
    let mut wechat_path_buf;
    let path = if let Some(wechat_path) = wechat_path {
        Path::new(wechat_path)
    } else {
        wechat_path_buf = dirs::document_dir().ok_or(anyhow!("fail to get document directory"))?;
        wechat_path_buf.push("WeChat Files");
        wechat_path_buf.as_path()
    };

    if !path.exists() || !path.is_dir() {
        return Err(anyhow!(format!(
            "指定的微信主目录不存在或者不是文件夹，请检查。{:?}",
            path.file_name()
        )));
    }
    for entity in read_dir(path)? {
        let entity = entity?;
        if entity.file_name() == "All Users"
            || entity.file_name() == "Applet"
            || entity.file_name() == "WMPF"
        {
            continue;
        }
        if entity.file_type()?.is_dir() {
            map.insert(
                entity
                    .file_name()
                    .into_string()
                    .map_err(|e| anyhow!(format!("fail to get String by OsString : {:?}", e)))?,
                entity.path(),
            );
        }
    }
    let mut path_buf = if let Some(path) = map.get(account) {
        let mut path_buf = path.clone();
        path_buf.push("Msg");
        path_buf.push("Multi");
        if path_buf.exists() && path_buf.is_dir() {
            return Err(anyhow!(format!(
                "未找到该用户名对应的文件夹. account: {account}"
            )));
        }
        path_buf
    } else {
        println!("请从下列中选择想要获取数据库的id:");
        let mut index_map = HashMap::new();
        for (index, file_name) in map.keys().enumerate() {
            let mut path_buf = map.get(file_name).ok_or(anyhow!("路径获取失败"))?.clone();
            path_buf.push("Msg");
            path_buf.push("Multi");
            if path_buf.exists() && path_buf.is_dir() {
                index_map.insert(index + 1, file_name);
                println!("[{}]:{}", index + 1, file_name);
            }
        }
        let key = loop {
            println!("请输入编号:");
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            let index = input.trim().parse::<usize>();
            if let Ok(index) = index {
                if let Some(file_name) = index_map.get(&index) {
                    break *file_name;
                } else {
                    println!("请输入正确的编号");
                }
            } else {
                println!("输入的不是数字，请输入数字编号");
                continue;
            }
        };
        let mut path_buf = map.get(key).ok_or(anyhow!("路径获取失败"))?.clone();
        path_buf.push("Msg");
        path_buf.push("Multi");
        path_buf
    };

    let save_path = Path::new(save_dir);
    if save_path.exists() && !save_path.is_dir() {
        return Err(anyhow!(
            "保存路径已经存在并且不是一个文件夹,save_path:{}",
            save_dir
        ));
    }
    if save_path.exists() {
        std::fs::remove_dir_all(save_path)?;
    }
    create_dir(save_path)?;

    for entry in read_dir(&path_buf)? {
        let entry = entry?;
        let file_name = entry
            .file_name()
            .into_string()
            .map_err(|e| anyhow!(format!("fail to get String by OsString : {:?}", e)))?;
        if entry.path().is_file()
            && file_name.starts_with("MSG")
            && (file_name.ends_with(".db")
                || file_name.ends_with(".db-wal")
                || file_name.ends_with(".db-shm"))
        {
            let mut in_file = File::open(entry.path())?;
            let mut save_path_buf = PathBuf::from(save_path);
            save_path_buf.push(file_name);
            let mut out_file = File::create(save_path_buf)?;
            std::io::copy(&mut in_file, &mut out_file)?;
        }
    }
    path_buf.pop();
    let file_name_list = ["MicroMsg", "PublicMsg", "Emotion"];
    let mut full_file_name_list = vec![];
    for filename in file_name_list {
        full_file_name_list.push(filename.to_owned() + ".db");
        full_file_name_list.push(filename.to_owned() + ".db-wal");
        full_file_name_list.push(filename.to_owned() + ".db-shm");
    }

    for entry in read_dir(&path_buf)? {
        let entry = entry?;
        let file_name = entry
            .file_name()
            .into_string()
            .map_err(|e| anyhow!(format!("fail to get String by OsString : {:?}", e)))?;
        if full_file_name_list.contains(&file_name) {
            let mut in_file = File::open(entry.path())?;
            let mut save_path_buf = PathBuf::from(save_path);
            save_path_buf.push(file_name);
            let mut out_file = File::create(save_path_buf)?;
            std::io::copy(&mut in_file, &mut out_file)?;
        }
    }
    Ok(())
}
