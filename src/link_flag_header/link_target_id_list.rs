#[derive(Debug, PartialEq)]
pub struct LinkTargetIDList {
    id_list_size: u16,
    // id_list: IdList,
    item_id_list: Vec<ItemId>,
}

// #[derive(Debug, PartialEq)]
// pub struct IdList {
//     // 2 bytes indicates end of ItemID, MUST be zero
//     // terminal_id: u16,
// }

#[derive(Debug, PartialEq)]
pub struct ItemId {
    item_id_size: u16,
    data: Vec<u8>,
}

impl LinkTargetIDList {
    pub fn size(&self) -> u16 {
        if self.id_list_size == 0 {
            return 0;
        }

        // + 2 because of size of id self.id_list_size
        self.id_list_size + 2
    }

    pub fn from_bytes(lnk_file: &[u8]) -> Result<Self, &'static str> {
        let mut id_list_size: u16 = 0;
        let id_list_size_iter = lnk_file.iter().take(2);
        if id_list_size_iter.clone().count() != 2 {
            return Err("Badly formated LinkTargetIDList");
        }

        for b in id_list_size_iter.rev() {
            id_list_size <<= 8;
            id_list_size += *b as u16;
        }

        let mut data_bytes = lnk_file.iter().skip(2).take(id_list_size as usize);
        let data_byte_count = data_bytes.clone().count();
        if data_byte_count != id_list_size as usize {
            return Err("Badly formated LinkTargetIDList");
        }

        let mut item_id_list: Vec<ItemId> = Vec::new();
        // println!("{:?}", data_bytes.clone().cloned());
        loop {
            let mut item_id_size: u16 = 0;
            let mut data = Vec::new();
            // Low Byte
            item_id_size += *data_bytes.next().unwrap() as u16;

            //High byte
            item_id_size += (*data_bytes.next().unwrap() as u16) << 8;

            if item_id_size == 0 {
                break;
            }

            for _ in 0..item_id_size - 2 {
                data.push(*data_bytes.next().unwrap());
            }

            item_id_list.push(ItemId { item_id_size, data });
        }
        // dbg!(lnk_file);

        Ok(Self {
            id_list_size,
            item_id_list,
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test() {
//         let id_list_size: [u8; 2] = [0xBD, 0x00];
//         let t: u16 = 0x00BD;
//         let t: u16 = 189;
//     }
// }
