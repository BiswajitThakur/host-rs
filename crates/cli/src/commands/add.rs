/*
use std::collections::HashSet;

use anyhow::{anyhow, Result};

use host_utils::{etc_host_reader, etc_write, host_path, read_file, HashList, H, R};
use host_utils::{Container, StoragePath};

pub(crate) fn allow(
    allow_args: Vec<&str>,
    etc_content: Vec<&str>,
    parent: &StoragePath,
) -> Result<()> {
    let mut h = HashSet::<H>::with_capacity(etc_content.len());
    etc_host_reader(&etc_content, &mut h);
    let args: HashList<H> = allow_args.into();
    let allow = read_file(parent.get_allow()).unwrap();
    let block = read_file(parent.get_block()).unwrap();
    let redirect = read_file(parent.get_redirect()).unwrap();
    let sources = read_file(parent.get_sources()).unwrap();

    let mut data = Container::init(
        allow.as_str(),
        block.as_str(),
        redirect.as_str(),
        sources.as_str(),
    );
    let mut r = HashSet::<&R>::with_capacity(data.get_redirect().len());
    for i in args.as_set() {
        h.remove(i);
        data.insert_allow(i.clone().into());
    }
    data.save(parent)?;
    for i in data.get_redirect().into_iter() {
        r.insert(i);
    }
    // etc_write(host_path(), (h, r), etc_content.into_iter())?;
    etc_write(host_path(), (h, r), etc_content.into_iter())?;
    Ok(())
}

pub(crate) fn block(
    block_args: Vec<&str>,
    etc_content: Vec<&str>,
    parent: &StoragePath,
) -> Result<()> {
    let mut h = HashSet::<H>::with_capacity(etc_content.len());
    //let mut r = HashList::<R>::with_capacity(512);
    etc_host_reader(&etc_content, &mut h);
    let args: HashList<H> = block_args.into();
    let allow = read_file(parent.get_allow()).unwrap();
    let block = read_file(parent.get_block()).unwrap();
    let redirect = read_file(parent.get_redirect()).unwrap();
    let sources = read_file(parent.get_sources()).unwrap();

    let mut data = Container::init(
        allow.as_str(),
        block.as_str(),
        redirect.as_str(),
        sources.as_str(),
    );

    let mut r = HashSet::<&R>::with_capacity(data.get_redirect().len());
    for i in args.as_set().into_iter() {
        h.insert(i.clone());
        data.insert_block(i.clone());
    }
    data.save(parent)?;
    for i in data.get_redirect().into_iter() {
        r.insert(i);
    }
    etc_write(host_path(), (h, r), etc_content.into_iter())?;
    Ok(())
}

pub(crate) fn redirect(
    args: Vec<&str>,
    etc_content: Vec<&str>,
    parent: &StoragePath,
) -> Result<()> {
    if args.len() % 2 != 0 {
        return Err(anyhow!("Invalid input..."));
    };
    let mut iter = args.iter();
    let mut rd = HashList::<R>::with_capacity(args.len() / 2);
    while let (Some(u), Some(v)) = (iter.next(), iter.next()) {
        rd.push(R::new(u, v));
    }
    let mut h = HashSet::<H>::with_capacity(etc_content.len());
    etc_host_reader(&etc_content, &mut h);
    //let args: HashList<H> = block_args.into();
    let allow = read_file(parent.get_allow()).unwrap();
    let block = read_file(parent.get_block()).unwrap();
    let redirect = read_file(parent.get_redirect()).unwrap();
    let sources = read_file(parent.get_sources()).unwrap();

    let mut data = Container::init(
        allow.as_str(),
        block.as_str(),
        redirect.as_str(),
        sources.as_str(),
    );

    let mut r = HashList::<&R>::with_capacity(rd.as_set().len());

    for i in rd.as_set().into_iter() {
        data.push_redirect(i.clone());
    }
    for i in data.get_redirect().iter() {
        r.push(i);
    }
    data.save(parent)?;
    etc_write(host_path(), (h, r.into()), etc_content.iter())?;
    Ok(())
}

pub(crate) fn sources(args: Vec<&str>, parent: &StoragePath) -> std::io::Result<()> {
    todo!()
}*/
