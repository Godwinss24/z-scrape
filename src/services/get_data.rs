use reqwest::Client;

pub async fn get_data(link: String) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();

    client.get(link)
    .header("accept", "application/json")
    .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36")
    .header("cookie", "intercom-id-nketzd4e=08a700ec-28d5-4b62-aa35-5aeb4ef33d55; intercom-device-id-nketzd4e=ac144407-ae02-42e4-aced-c87e28611f3c; _fbp=fb.1.1759828535786.242667695334116567; cookie-config={%22analytics%22:true%2C%22marketing%22:true%2C%22functional%22:true}; mp_331e7ed57ec193ae7fde9e90b8ef68d4_mixpanel=%7B%22distinct_id%22%3A%22%24device%3Af4d0e533-53e6-4f9b-8d42-9475c25399a2%22%2C%22%24device_id%22%3A%22f4d0e533-53e6-4f9b-8d42-9475c25399a2%22%2C%22%24initial_referrer%22%3A%22%24direct%22%2C%22%24initial_referring_domain%22%3A%22%24direct%22%2C%22mps%22%3A%7B%7D%2C%22mpso%22%3A%7B%22%24initial_referrer%22%3A%22%24direct%22%2C%22%24initial_referring_domain%22%3A%22%24direct%22%7D%2C%22mpus%22%3A%7B%7D%2C%22mpa%22%3A%7B%7D%2C%22mpu%22%3A%7B%7D%2C%22mpr%22%3A%5B%5D%2C%22__mpap%22%3A%5B%5D%7D; connect.sid=s%3AM_JpTbEjKLo-zOItpPtkqdUZF_hPyb54.UW5L%2Bmsav30tlHKMggzZi3Zi17cT5ozFjrj0nL%2FahDU; access_token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiJjNzFkYWM1OS0yNmU2LTRkYzctOWQ2My1mNzYyZDRmYTNhZmYiLCJhY2NvdW50VHlwZSI6ImVtYWlsIiwiZW1haWwiOiJtYXlhamFuZTQ2M0BnbWFpbC5jb20iLCJsYXN0RW1haWxDaGVjayI6MTc3MDM5NDE3ODY0OCwiaWF0IjoxNzcwMzk0MTc4LCJleHAiOjE3NzI5ODYxNzh9.4hEdDLVW4teiqVvUZuWltE1gCZQ6LMvgz68dicQWRo8; user_metadata=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiJjNzFkYWM1OS0yNmU2LTRkYzctOWQ2My1mNzYyZDRmYTNhZmYiLCJpYXQiOjE3NzAzOTQxNzgsImV4cCI6MTc3Mjk4NjE3OH0.tw_X3Xqu8ouooSdSRDck4J6Jx3j8tlJojcO8biObq58; intercom-session-nketzd4e=ajZTWklnVzh4UGZDSWVYekRPbWZJK1MxcU5UY2o1TGhRVTVjTDlOekFyWG15SXU0V3I2MFlOb2ZzOXZCUGpiY1ZTYzJOZUlPRGtVNlVkYVZldzdZa2NJRVFnSDRMZ2JVdDFzR2k0dXlYck09LS1rSVhNd0RVZXM2UG5IRmJIOEViTHVRPT0=--ad277d64a7e3961bfabf6073841a218b63ee3b7b")
    .header("referer", "https://zealy.io/")
    .header("origin", "https://zealy.io")
    .send().await
}
