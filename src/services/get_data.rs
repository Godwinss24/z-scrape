use reqwest::Client;

pub async fn get_data(link: String) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();

    client.get(link)
    .header("accept", "application/json")
    .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36")
    .header("cookie", "_fbp=fb.1.1757484837783.791316301995086529; intercom-id-nketzd4e=7b188e9b-37cd-4eb2-b237-03b7c1bd12be; intercom-device-id-nketzd4e=a417d689-3590-4d3d-a45e-8cd546b064f6; _tt_enable_cookie=1; _ttp=01K4S47S480VY8H1MNZZMZ0HFR_.tt.1; ttcsid=1764220788689::L-jCFvc7s-sRze7XFXRC.7.1764220854788.0; ttcsid_CO6OII3C77UAL9O5M6RG=1764220788687::OkMTKUWJQTij_N21C0IV.7.1764220854789.0; cookie-config={%22analytics%22:true%2C%22marketing%22:true%2C%22functional%22:true}; access_token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiIyNTM1MjJjZS0wYjQzLTRlZTctOWQxMy0yNGY4OTUyNDcyYzEiLCJhY2NvdW50VHlwZSI6ImVtYWlsIiwiZW1haWwiOiJhenV6ZTIyQGdvZHdpbnMud29yay5nZCIsImxhc3RFbWFpbENoZWNrIjoxNzY2OTkzODQyMDgwLCJpYXQiOjE3NjY5OTM4NDIsImV4cCI6MTc2OTU4NTg0Mn0.QybHxf7qJ-tUAQ7OaoZ956yrGZkearQmwY55b-eGGqs; user_metadata=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiIyNTM1MjJjZS0wYjQzLTRlZTctOWQxMy0yNGY4OTUyNDcyYzEiLCJpYXQiOjE3NjY5OTM4NDIsImV4cCI6MTc2OTU4NTg0Mn0.Ucx9PunJhTHzwE5R9bgXaedwmorpT394q8mdGIiJELA; subdomain=root; connect.sid=s%3AG67JLuGcnm4kB_OMB2zNiXAymTvYiWnj.gd%2Bve7b4n31rXq0RTgLwYTTK0Z%2Bc5908LhuB1seKnwI; intercom-session-nketzd4e=U2VWa3RsaW5SQzVoeWR3L3FnV1VLKzRjV2RHRmIzNG9sZ2VyQk9JdG5HSW50YURrV2IxamV0UWdLMUlTOHZHZzU2THBEZXJlbDlLaU1NTmVSM1hDY2NRcjlVeVk2NFprMGFkV1dtbHNqUlU9LS1NbWRsSlpoVisxZ1cvNHNRVzBQOXVBPT0=--79d18de0d1a782e9a9c28d0c1416b7b7ae89247d; mp_331e7ed57ec193ae7fde9e90b8ef68d4_mixpanel=%7B%22distinct_id%22%3A%22%24device%3A99943d24-7b13-4afb-8710-8b4a85df60e8%22%2C%22%24device_id%22%3A%2299943d24-7b13-4afb-8710-8b4a85df60e8%22%2C%22%24initial_referrer%22%3A%22%24direct%22%2C%22%24initial_referring_domain%22%3A%22%24direct%22%2C%22mps%22%3A%7B%7D%2C%22mpso%22%3A%7B%22%24initial_referrer%22%3A%22%24direct%22%2C%22%24initial_referring_domain%22%3A%22%24direct%22%7D%2C%22mpus%22%3A%7B%7D%2C%22mpa%22%3A%7B%7D%2C%22mpu%22%3A%7B%7D%2C%22mpr%22%3A%5B%5D%2C%22__mpap%22%3A%5B%5D%7D")
    .header("referer", "https://zealy.io/")
    .header("origin", "https://zealy.io")
    .send().await
}
