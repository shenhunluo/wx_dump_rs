use crate::{
    util::{get_all_memory_by_handle, get_data_by_real_addr, get_modules},
    wx_util::WeChatInfo,
};

pub fn memory_search(
    wechat_info: &WeChatInfo,
    data: &[u8],
    real_addr: bool,
) -> anyhow::Result<Vec<usize>> {
    let vec = get_data_by_real_addr(
        wechat_info.process.th32ProcessID,
        wechat_info.module.modBaseAddr as usize,
        wechat_info.module.modBaseSize as usize,
    )?;
    let r = (0..vec.len() - data.len())
        .filter(|&i| &vec[i..i + data.len()] == data)
        .map(|i| {
            if real_addr {
                wechat_info.module.modBaseAddr as usize + i
            } else {
                i
            }
        })
        .collect();
    Ok(r)
}

pub fn memory_search_from_wechat_all_modules(
    wechat_info: &WeChatInfo,
    data: &[u8],
    real_addr: bool,
    show_no_found_info: bool,
    show_error_info: bool,
) -> anyhow::Result<()> {
    for module in get_modules(wechat_info.process)? {
        let vec = get_data_by_real_addr(
            wechat_info.process.th32ProcessID,
            module.modBaseAddr as usize,
            module.modBaseSize as usize,
        );
        match vec {
            Ok(vec) => {
                let r: Vec<usize> = (0..vec.len() - data.len())
                    .filter(|&i| &vec[i..i + data.len()] == data)
                    .map(|i| {
                        if real_addr {
                            module.modBaseAddr as usize + i
                        } else {
                            i
                        }
                    })
                    .collect();
                if r.len() > 0 {
                    println!(
                        "module: {}",
                        String::from_utf8(
                            module.szModule.split(|n| *n == 0).next().unwrap().to_vec()
                        )?
                    );
                    println!("{:?}", r);
                } else {
                    if show_no_found_info {
                        println!("在 {} 中未找到想要搜索的数据。开始位置：{},结束位置：{}, 长度：{}, vec 长度：{}",String::from_utf8(module.szModule.split(|n| *n == 0).next().unwrap().to_vec())?,module.modBaseAddr as usize,module.modBaseAddr as usize + module.modBaseSize as usize, module.modBaseSize, vec.len());
                    }
                }
            }
            Err(err) => {
                if show_error_info {
                    println!(
                        "获取内存失败。module: {}。err: {err:?}",
                        String::from_utf8(
                            module.szModule.split(|n| *n == 0).next().unwrap().to_vec()
                        )?
                    );
                    println!(
                        "addr start: {:?},size: {:?},end: {:?}",
                        module.modBaseAddr as usize,
                        module.modBaseSize as usize,
                        module.modBaseAddr as usize + module.modBaseSize as usize
                    );
                }
                continue;
            }
        }
    }
    Ok(())
}

pub fn memory_search_from_wechat_all_data(
    wechat_info: &WeChatInfo,
    data: &[u8],
    real_addr: bool,
    show_no_found_info: bool,
    show_error_info: bool,
) -> anyhow::Result<()> {
    for (base_addr, size) in get_all_memory_by_handle(&wechat_info.handle)? {
        let vec = get_data_by_real_addr(wechat_info.process.th32ProcessID, base_addr, size);
        match vec {
            Ok(vec) => {
                let r: Vec<usize> = (0..vec.len() - data.len())
                    .filter(|&i| &vec[i..i + data.len()] == data)
                    .map(|i| if real_addr { base_addr + i } else { i })
                    .collect();
                if r.len() > 0 {
                    println!("base_addr: {}", base_addr);
                    println!("{:?}", r);
                } else {
                    if show_no_found_info {
                        println!(
                            "未找到想要搜索的数据。开始位置：{},结束位置：{}, 长度：{}, vec 长度：{}",
                            base_addr,
                            base_addr + size,
                            size,
                            vec.len()
                        );
                    }
                }
            }
            Err(err) => {
                if show_error_info {
                    println!("获取内存失败。base_addr: {base_addr}。 size: {size}, err: {err:?}");
                }
                continue;
            }
        }
    }
    Ok(())
}

pub fn get_memory(
    wechat_info: &WeChatInfo,
    index: usize,
    len: usize,
    real_addr: bool,
) -> anyhow::Result<Vec<u8>> {
    let vec = if real_addr {
        get_data_by_real_addr(wechat_info.process.th32ProcessID, index, len)?
    } else {
        get_data_by_real_addr(
            wechat_info.process.th32ProcessID,
            wechat_info.module.modBaseAddr as usize + index,
            len,
        )?
    };
    Ok(vec)
}
