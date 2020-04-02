use crate::errors::*;
use crate::traits::{Close, High, Low, Open, Volume};

#[derive(Debug, Clone)]
pub struct DataItem {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

impl DataItem {
    pub fn builder() -> DataItemBuilder {
        DataItemBuilder::new()
    }
}

impl Open for DataItem {
    fn open(&self) -> f64 {
        self.open
    }
}

impl High for DataItem {
    fn high(&self) -> f64 {
        self.high
    }
}

impl Low for DataItem {
    fn low(&self) -> f64 {
        self.low
    }
}

impl Close for DataItem {
    fn close(&self) -> f64 {
        self.close
    }
}

impl Volume for DataItem {
    fn volume(&self) -> f64 {
        self.volume
    }
}

pub struct DataItemBuilder {
    open: Option<f64>,
    high: Option<f64>,
    low: Option<f64>,
    close: Option<f64>,
    volume: Option<f64>,
}

impl DataItemBuilder {
    pub fn new() -> Self {
        Self {
            open: None,
            high: None,
            low: None,
            close: None,
            volume: None,
        }
    }

    pub fn open(mut self, val: f64) -> Self {
        self.open = Some(val);
        self
    }

    pub fn high(mut self, val: f64) -> Self {
        self.high = Some(val);
        self
    }

    pub fn low(mut self, val: f64) -> Self {
        self.low = Some(val);
        self
    }

    pub fn close(mut self, val: f64) -> Self {
        self.close = Some(val);
        self
    }

    pub fn volume(mut self, val: f64) -> Self {
        self.volume = Some(val);
        self
    }

    pub fn build(self) -> Result<DataItem> {
        if let (Some(open), Some(high), Some(low), Some(close), Some(volume)) =
            (self.open, self.high, self.low, self.close, self.volume)
        {
            // validate
            if low <= open
                && low <= close
                && low <= high
                && high >= open
                && high >= close
                && volume >= 0.0
                && low >= 0.0
            {
                let item = DataItem {
                    open,
                    high,
                    low,
                    close,
                    volume,
                };
                Ok(item)
            } else {
                Err(Error::from_kind(ErrorKind::DataItemInvalid))
            }
        } else {
            Err(Error::from_kind(ErrorKind::DataItemIncomplete))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        fn assert_valid((open, high, low, close, volume): (f64, f64, f64, f64, f64)) {
            let result = DataItem::builder()
                .open(open)
                .high(high)
                .low(low)
                .close(close)
                .volume(volume)
                .build();

            assert!(result.is_ok());
            println!("{:#?}", result.unwrap());
        }

        fn assert_invalid((open, high, low, close, volume): (f64, f64, f64, f64, f64)) {
            let result = DataItem::builder()
                .open(open)
                .high(high)
                .low(low)
                .close(close)
                .volume(volume)
                .build();
            assert!(result.is_err());
        }

        let valid_records = vec![
            // open, high, low , close, volume
            (20.0, 25.0, 15.0, 21.0, 7500.0),
            (10.0, 10.0, 10.0, 10.0, 10.0),
            (0.0, 0.0, 0.0, 0.0, 0.0),
        ];
        for record in valid_records {
            assert_valid(record)
        }

        let invalid_records = vec![
            // open, high, low , close, volume
            (-1.0, 25.0, 15.0, 21.0, 7500.0),
            (20.0, -1.0, 15.0, 21.0, 7500.0),
            (20.0, 25.0, -1.0, 21.0, 7500.0),
            (20.0, 25.0, 15.0, -1.0, 7500.0),
            (20.0, 25.0, 15.0, 21.0, -1.0),
            (14.9, 25.0, 15.0, 21.0, 7500.0),
            (25.1, 25.0, 15.0, 21.0, 7500.0),
            (20.0, 25.0, 15.0, 14.9, 7500.0),
            (20.0, 25.0, 15.0, 25.1, 7500.0),
            (20.0, 15.0, 25.0, 21.0, 7500.0),
        ];
        for record in invalid_records {
            assert_invalid(record)
        }
    }
}
