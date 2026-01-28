use reqwest::Client;

pub async fn get_data(link: String) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();

    client.get(link)
    .header("accept", "application/json")
    .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36")
    .header("cookie", "intercom-id-nketzd4e=08a700ec-28d5-4b62-aa35-5aeb4ef33d55; intercom-device-id-nketzd4e=ac144407-ae02-42e4-aced-c87e28611f3c; _fbp=fb.1.1759828535786.242667695334116567; cookie-config={%22analytics%22:true%2C%22marketing%22:true%2C%22functional%22:true}; access_token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiJjNzFkYWM1OS0yNmU2LTRkYzctOWQ2My1mNzYyZDRmYTNhZmYiLCJhY2NvdW50VHlwZSI6ImVtYWlsIiwiZW1haWwiOiJtYXlhamFuZTQ2M0BnbWFpbC5jb20iLCJsYXN0RW1haWxDaGVjayI6MTc2Nzc5MjAwNzQzNywiaWF0IjoxNzY3NzkyMDA3LCJleHAiOjE3NzAzODQwMDd9.sCUpKzF_0umwgJfysIQehyhyCIKokzS4kCfegTmP-gU; user_metadata=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiJjNzFkYWM1OS0yNmU2LTRkYzctOWQ2My1mNzYyZDRmYTNhZmYiLCJpYXQiOjE3Njc3OTIwMDcsImV4cCI6MTc3MDM4NDAwN30.K8frRJV26apqhoArcplKfG2XOP-hf89m6_UnaQC4SKc; connect.sid=s%3ApP-FhETWy7FL2PcD0wDLztZlXN-yc3Yc.gr0ZSKY%2FUNi7PhhQ0In0LJ28vtAdYI%2Btnn2t9N0eREo; intercom-session-nketzd4e=VzZnMDlrUG1jS0gyMGhteWIzZkV2bXNxM3N4YmR0S2JEZEkxQ21HdXRvMlBLSTFPdlRYU3JTcnQ5NXlTY0FhZk00RTVIay9vdURmOEdrVnVGOFhuMzVwQ1orNGd5Sit2bjJ1STZBTWhEdHM9LS1WVjZlYk5QbENZazR0ZGtwa2VBanlBPT0=--b5e329836bbc9b0eb0e37ae95a737e2d85db1c94; mp_331e7ed57ec193ae7fde9e90b8ef68d4_mixpanel=%7B%22distinct_id%22%3A%22%24device%3Af4d0e533-53e6-4f9b-8d42-9475c25399a2%22%2C%22%24device_id%22%3A%22f4d0e533-53e6-4f9b-8d42-9475c25399a2%22%2C%22%24initial_referrer%22%3A%22%24direct%22%2C%22%24initial_referring_domain%22%3A%22%24direct%22%2C%22mps%22%3A%7B%7D%2C%22mpso%22%3A%7B%22%24initial_referrer%22%3A%22%24direct%22%2C%22%24initial_referring_domain%22%3A%22%24direct%22%7D%2C%22mpus%22%3A%7B%7D%2C%22mpa%22%3A%7B%7D%2C%22mpu%22%3A%7B%7D%2C%22mpr%22%3A%5B%5D%2C%22__mpap%22%3A%5B%5D%7D")
    .header("referer", "https://zealy.io/")
    .header("origin", "https://zealy.io")
    .send().await
}
