use crate::app::App;

impl App {
    pub fn next(&mut self) {
        let data_len = if let Some(kabkota_filtered_data) = &self.kabkota_items_filter {
            &kabkota_filtered_data.len()
        } else if let Some(kabkota_data) = &self.kabkota_items {
            &kabkota_data.len()
        } else if let Some(province_filtered_data) = &self.province_items_filter {
            &province_filtered_data.len()
        } else {
            &self.province_items.len()
        };

        let i = match self.province_state.selected() {
            Some(i) => {
                if i >= data_len - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.province_state.select(Some(i));
    }

    pub fn prev(&mut self) {
        let data_len = if let Some(kabkota_filtered_data) = &self.kabkota_items_filter {
            &kabkota_filtered_data.len()
        } else if let Some(kabkota_data) = &self.kabkota_items {
            &kabkota_data.len()
        } else if let Some(province_filtered_data) = &self.province_items_filter {
            &province_filtered_data.len()
        } else {
            &self.province_items.len()
        };

        let i = match self.province_state.selected() {
            Some(i) => {
                if i == 0 {
                    data_len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.province_state.select(Some(i));
    }

    pub fn enter(&mut self) {
        let (data, status) = if let Some(kabkota_filtered_data) = &self.kabkota_items_filter {
            (kabkota_filtered_data, true)
        } else if let Some(kabkota_data) = &self.kabkota_items {
            (kabkota_data, true)
        } else if let Some(province_filtered_data) = &self.province_items_filter {
            (province_filtered_data, false)
        } else {
            (&self.province_items, false)
        };

        if let Some(index) = self.province_state.selected() {
            if status {
                self.kabkota_selected = Some(data[index].clone());
            } else {
                self.province_selected = Some(data[index].clone());
            }
        }
    }
}
