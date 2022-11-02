server: cd server && cargo watch -x run
cypress: cd browser-tests && yarn cypress open
mock_bing: httpmock --mock-files-dir server/src/external_apis/mocks/bing_news --port 9700
tailwind: tailwindcss -i frontend/src/tailwind.css -o frontend/pkg/tailwind.css --watch
