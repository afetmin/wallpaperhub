use chrono::NaiveDate;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

const BING_URL: &str = "https://www.bing.com/HPImageArchive.aspx?&format=js&uhd=1&uhdwidth=3840&uhdheight=2160";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BingImage {
    pub url: String,
    pub urlbase: String,
    pub copyright: String,
    pub copyrightlink: String,
    pub title: String,
    pub startdate: String,
    pub enddate: String,
}

impl BingImage {
    pub fn get_full_url(&self) -> String {
        format!("https://www.bing.com{}", self.url)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tooltips {
    // 正在加载...
    loading: String,
    // 此图片不能下载用作壁纸。
    walle: String,
    // 下载今日美图。仅限用作桌面壁纸。
    walls: String,
}

pub struct UrlParams {
    pub(crate) index: u8,
    pub(crate) number: u8,
    pub(crate) country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BingImageRes {
    pub images: Vec<BingImage>,
    pub tooltips: Tooltips,
}

impl BingImageRes {
    pub async fn new(url_params: UrlParams) -> Result<BingImageRes, reqwest::Error> {
        let bing_url = get_bing_url(url_params);
        println!("Bing url =====> {}", bing_url);
        let response = reqwest::get(bing_url).await?.json().await?;
        Ok(response)
    }
}

lazy_static! {
    static ref CACHED_BING_IMAGE_RES: Mutex<Option<CachedBingImageRes>> = Mutex::new(None);
}

#[derive(Debug)]
struct CachedBingImageRes {
    data: BingImageRes,
    end_date: NaiveDate,
}

pub async fn get_cached_bing_image_res(number: u8) -> Result<BingImageRes, reqwest::Error> {
    let mut cache = CACHED_BING_IMAGE_RES.lock().await;
    let today = chrono::Local::now().naive_local();
        println!("Cached data ========> {:?}", *cache);
    if let Some(CachedBingImageRes { data, end_date }) = &*cache {
        if today.date() <= *end_date {
            return Ok(data.clone());
        }
    }

    let url_params = UrlParams {
        index: 0,
        number,
        country: Some("en-US".to_string()),
    };
    let new_data = BingImageRes::new(url_params).await?;
    *cache = Some(CachedBingImageRes {
        data: new_data.clone(),
        end_date: today.date(),
    });

    Ok(new_data)
}

pub fn get_bing_url(url_params: UrlParams) -> String {
    let UrlParams { index, number, country } = url_params;
    let mut url = format!("{}&idx={}&n={}", BING_URL, index, number);
    if let Some(country) = country {
        url.push_str(&format!("&mkt={}", country));
    }
    url
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bing_url() {
        let url_params = UrlParams {
            index: 0,
            number: 1,
            country: Some("en-US".to_string()),
        };
        let url = get_bing_url(url_params);

        println!("{}", url);
        assert_eq!(url, "https://www.bing.com/HPImageArchive.aspx?&format=js&uhd=1&uhdwidth=3840&uhdheight=2160&idx=0&n=1&mkt=en-US");
    }

    #[tokio::test]
    async fn test_bing_image_res() {
        let url_params = UrlParams {
            index: 0,
            number: 8,
            country: Some("en-US".to_string()),
        };
        let bing_image_res = BingImageRes::new(url_params).await.unwrap();
        println!("{:?} {:?}", bing_image_res, bing_image_res.images.len());
    }
}
