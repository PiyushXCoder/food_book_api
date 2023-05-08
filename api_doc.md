# Food Book API Documentation

> Note: All routes starting with /api/o doesn't require authentication while all with /api/s does require Bearer Authentication

## User And Authentication

### Initialise create account

```
POST /api/o/user?first_name={string}&last_name={string}&email={string}&avatar={string}&password={string}
```

```bash
curl -v -X POST "http://localhost:8080/api/o/user?first_name=Jay&last_name=Singh&email=derog78811@soombo.com&password=toor"
```

```bash
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
> POST /api/o/user?first_name=Jay&last_name=Singh&email=derog78811@soombo.com&password=toor HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/8.0.1
> Accept: */*
> 
< HTTP/1.1 201 Created
< content-length: 41
< date: Sat, 06 May 2023 05:36:07 GMT
< 
* Connection #0 to host localhost left intact
{"message":"Account creation email sent"}⏎   
```

### Create account

```
PUT /api/o/user?token={string}
```

```bash
curl -v -X PUT "http://localhost:8080/api/o/user?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJmaXJzdF9uYW1lIjoiSmF5IiwibGFzdF9uYW1lIjoiU2luZ2giLCJlbWFpbCI6InJhY294ZXM1OTlAc2Flb2lsLmNvbSIsInBhc3N3b3JkIjoidG9vciIsImp0aSI6IjM1YTk0MDZlLWEwZjEtNDlkNy1hNWQ4LTllYWIyNjVjNmQ5ZCIsImV4cCI6MTY4MzM1NTQxM30.CfARK6BdWLgKOv2vZF3Qc-R2f7Isqw1XxIkXI-XIKrg"
```

```bash
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
> PUT /api/o/user?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJmaXJzdF9uYW1lIjoiSmF5IiwibGFzdF9uYW1lIjoiU2luZ2giLCJlbWFpbCI6InJhY294ZXM1OTlAc2Flb2lsLmNvbSIsInBhc3N3b3JkIjoidG9vciIsImp0aSI6IjM1YTk0MDZlLWEwZjEtNDlkNy1hNWQ4LTllYWIyNjVjNmQ5ZCIsImV4cCI6MTY4MzM1NTQxM30.CfARK6BdWLgKOv2vZF3Qc-R2f7Isqw1XxIkXI-XIKrg HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/8.0.1
> Accept: */*
> 
< HTTP/1.1 201 Created
< content-length: 32
< date: Sat, 06 May 2023 05:44:56 GMT
< 
* Connection #0 to host localhost left intact
{"message":"Account is created"}⏎ 

```

### Login

```
POST /api/o/login?email={string}&password={string}
```

```bash
curl -v -X POST "http://localhost:8080/api/o/login?email=racoxes599@saeoil.com&password=toor"
```

```bash
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
> POST /api/o/login?email=racoxes599@saeoil.com&password=toor HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/8.0.1
> Accept: */*
> 
< HTTP/1.1 201 Created
< content-length: 267
< date: Sat, 06 May 2023 05:49:26 GMT
< 
* Connection #0 to host localhost left intact
{"message":"Logged in","token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjozLCJmaXJzdF9uYW1lIjoiSmF5IiwibGFzdF9uYW1lIjoiU2luZ2giLCJqdGkiOiI4ZDhmMDRjMC1lZjA3LTQ3OTQtYmUwOC1jMzFkYjAzNDY1NzYiLCJleHAiOjE2ODM5NTY5Njd9.7aLmrGVkuFhTlVGowyLkBEgqyRaOgdhWV2VnBgjBUSY"}⏎  
```

### Get User Info

```
GET /api/s/user/{user_id}
```

```bash
curl -v -H "Authorization: Bearer $TOKEN" -X GET "http://localhost:8080/api/s/user/3"
```

```bash
Note: Unnecessary use of -X or --request, GET is already inferred.
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
> GET /api/s/user/3 HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/8.0.1
> Accept: */*
> Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjozLCJmaXJzdF9uYW1lIjoiSmF5IiwibGFzdF9uYW1lIjoiU2luZ2giLCJqdGkiOiI4ZDhmMDRjMC1lZjA3LTQ3OTQtYmUwOC1jMzFkYjAzNDY1NzYiLCJleHAiOjE2ODM5NTY5Njd9.7aLmrGVkuFhTlVGowyLkBEgqyRaOgdhWV2VnBgjBUSY
> 
< HTTP/1.1 200 OK
< content-length: 79
< date: Mon, 08 May 2023 07:36:09 GMT
< 
* Connection #0 to host localhost left intact
{"id":1,"first_name":"Jay","last_name":"Singh","email":"racoxes599@saeoil.com","avatar":null}⏎
```

## Post

### Create post


```
POST /api/s/post

Content-type: application/json

Payload:
    {
        "heading": {string}, 
        "sub_heading": {string}, 
        "caption": {string}, 
        "cooking_duration": {string in format "00:00:00"}, 
        "tags": [string], 
        "visuals": [string], 
        "ingredients": [string], 
        "steps": [string]
    } 
```

```bash
curl -v -XPOST -H "Authorization: Bearer $TOKEN" -H "Content-type: application/json" -d '{"heading": "Gol Gappa", 
                   "sub_heading": "Mast wala", 
                   "caption": "Easy recipie", 
                   "cooking_duration": "00:00:00", 
                   "tags": ["easy"], 
                   "visuals": [], 
                   "ingredients": ["aata", "mirch"], 
                   "steps": ["pahla", "doosra"]}' 'http://localhost:8080/api/s/post'
```

```bash
Note: Unnecessary use of -X or --request, POST is already inferred.
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
> POST /api/s/post HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/8.0.1
> Accept: */*
> Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoxLCJmaXJzdF9uYW1lIjoiSmF5IiwibGFzdF9uYW1lIjoiU2luZ2giLCJhdmF0YXIiOiIiLCJqdGkiOiI1MmZiNDNlZC0xODk1LTQ4ZDAtOWFhOC1hNmNkYTU0MWI1YjciLCJleHAiOjE2ODQxMzg0OTJ9.A4ZJIALOHTdgEtUDiJoTwZLuriZkyQf4dobBNdG3mp4
> Content-type: application/json
> Content-Length: 215
> 
< HTTP/1.1 201 Created
< content-length: 36
< date: Mon, 08 May 2023 08:50:18 GMT
< 
* Connection #0 to host localhost left intact
{"id":2,"message":"Post is created"}⏎ 
```

### Update Post

```
PUT /api/s/post/{post_id}

Content-type: application/json

Payload:
    {
        "heading": {string}, 
        "sub_heading": {string}, 
        "caption": {string}, 
        "cooking_duration": {string in format "00:00:00"}, 
        "tags": [string], 
        "visuals": [string], 
        "ingredients": [string], 
        "steps": [string]
    } 
```
```bash
curl -v -XPUT -H "Authorization: Bearer $TOKEN" -H "Content-type: application/json" -d '{"heading": "Gol Gappa", 
                   "sub_heading": "Mast wala", 
                   "caption": "Easy recipie", 
                   "cooking_duration": "00:00:00", 
                   "tags": ["easy"], 
                   "visuals": [], 
                   "ingredients": ["aata", "mirch"], 
                   "steps": ["pahla", "doosra"]}' 'http://localhost:8080/api/s/post/2'
```

```bash
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
> PUT /api/s/post/2 HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/8.0.1
> Accept: */*
> Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoxLCJmaXJzdF9uYW1lIjoiSmF5IiwibGFzdF9uYW1lIjoiU2luZ2giLCJhdmF0YXIiOiIiLCJqdGkiOiI1MmZiNDNlZC0xODk1LTQ4ZDAtOWFhOC1hNmNkYTU0MWI1YjciLCJleHAiOjE2ODQxMzg0OTJ9.A4ZJIALOHTdgEtUDiJoTwZLuriZkyQf4dobBNdG3mp4
> Content-type: application/json
> Content-Length: 215
> 
< HTTP/1.1 201 Created
< content-length: 29
< date: Mon, 08 May 2023 08:56:42 GMT
< 
* Connection #0 to host localhost left intact
{"message":"Post is updated"}⏎  
```

### Get Post Info

```
GET /api/o/post/{post_id}

```

```bash
curl -v  -X GET "localhost:8080/api/o/post/2"
```

```bash
Note: Unnecessary use of -X or --request, GET is already inferred.
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
> GET /api/o/post/2 HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/8.0.1
> Accept: */*
> 
< HTTP/1.1 200 OK
< content-length: 351
< date: Mon, 08 May 2023 10:02:42 GMT
< 
* Connection #0 to host localhost left intact
[{"id":2,"user_id":1,"heading":"Gol Gappa","sub_heading":"Mast wala","caption":"Easy recipie","cooking_duration":"00:00:00","tags":["easy"],"visuals":[],"ingredients":["aata","mirch"],"steps":["pahla","doosra"],"likes_count":0,"comments_count":0,"created_at":"2023-05-08T08:50:18.610226"},{"id":1,"first_name":"Jay","last_name":"Singh","avatar":null}]⏎      
```

### Delete Post

```
DELETE /api/s/post/{post_id}
```

```bash
curl -v -H "Authorization: Bearer $TOKEN" -X DELETE "localhost:8080/api/s/post/2"
```

```bash
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
> DELETE /api/s/post/2 HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/8.0.1
> Accept: */*
> Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoxLCJmaXJzdF9uYW1lIjoiSmF5IiwibGFzdF9uYW1lIjoiU2luZ2giLCJhdmF0YXIiOiIiLCJqdGkiOiI1MmZiNDNlZC0xODk1LTQ4ZDAtOWFhOC1hNmNkYTU0MWI1YjciLCJleHAiOjE2ODQxMzg0OTJ9.A4ZJIALOHTdgEtUDiJoTwZLuriZkyQf4dobBNdG3mp4
> 
< HTTP/1.1 201 Created
< content-length: 29
< date: Mon, 08 May 2023 10:39:40 GMT
< 
* Connection #0 to host localhost left intact
{"message":"Post is deleted"}⏎              
```

### List Posts

```
GET /api/s/posts?limit={int}&offset={int - set 0 for start}&user_id={int - optional}&name={string - optional}
```

```bash
curl -v -H "Authorization: Bearer $TOKEN" -X GET "localhost:8080/api/s/posts?limit=5&offset=0&user_id=1&name=G"
```

```bash
Note: Unnecessary use of -X or --request, GET is already inferred.
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
> GET /api/s/posts?limit=5&offset=0&user_id=1&name=G HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/8.0.1
> Accept: */*
> Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoxLCJmaXJzdF9uYW1lIjoiSmF5IiwibGFzdF9uYW1lIjoiU2luZ2giLCJhdmF0YXIiOiIiLCJqdGkiOiI1MmZiNDNlZC0xODk1LTQ4ZDAtOWFhOC1hNmNkYTU0MWI1YjciLCJleHAiOjE2ODQxMzg0OTJ9.A4ZJIALOHTdgEtUDiJoTwZLuriZkyQf4dobBNdG3mp4
> 
< HTTP/1.1 200 OK
< content-length: 270
< date: Mon, 08 May 2023 10:01:03 GMT
< 
* Connection #0 to host localhost left intact
[[{"id":2,"user_id":1,"heading":"Gol Gappa","sub_heading":"Mast wala","cooking_duration":"00:00:00","tags":["easy"],"visuals":[],"likes_count":0,"comments_count":0,"created_at":"2023-05-08T08:50:18.610226"},{"id":1,"first_name":"Jay","last_name":"Singh","avatar":null}]]⏎ 
```

### Add Comment

```
POST /api/s/post/{post_id}/comment?note={string}
```

```bash
curl -v -H "Authorization: Bearer $TOKEN" -X POST "localhost:8080/api/s/post/2/comment?note=Wow"
```

```bash
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
> POST /api/s/post/2/comment?note=Wow HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/8.0.1
> Accept: */*
> Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoxLCJmaXJzdF9uYW1lIjoiSmF5IiwibGFzdF9uYW1lIjoiU2luZ2giLCJhdmF0YXIiOiIiLCJqdGkiOiI1MmZiNDNlZC0xODk1LTQ4ZDAtOWFhOC1hNmNkYTU0MWI1YjciLCJleHAiOjE2ODQxMzg0OTJ9.A4ZJIALOHTdgEtUDiJoTwZLuriZkyQf4dobBNdG3mp4
> 
< HTTP/1.1 201 Created
< content-length: 37
< date: Mon, 08 May 2023 10:09:33 GMT
< 
* Connection #0 to host localhost left intact
{"id":1,"message":"Comment is added"}⏎              
```

### Delete Comment

```
DELETE /api/s/post/comment/{comment_id}
```

```bash
curl -v -H "Authorization: Bearer $TOKEN" -X DELETE "localhost:8080/api/s/post/comment/1"
```

```bash
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
> DELETE /api/s/post/comment/1 HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/8.0.1
> Accept: */*
> Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoxLCJmaXJzdF9uYW1lIjoiSmF5IiwibGFzdF9uYW1lIjoiU2luZ2giLCJhdmF0YXIiOiIiLCJqdGkiOiI1MmZiNDNlZC0xODk1LTQ4ZDAtOWFhOC1hNmNkYTU0MWI1YjciLCJleHAiOjE2ODQxMzg0OTJ9.A4ZJIALOHTdgEtUDiJoTwZLuriZkyQf4dobBNdG3mp4
> 
< HTTP/1.1 201 Created
< content-length: 32
< date: Mon, 08 May 2023 10:24:07 GMT
< 
* Connection #0 to host localhost left intact
{"message":"Comment is deleted"}⏎          
```

### List Comments

```
GET /api/s/post/{post_id}/comments?limit={int}&offset={int - set zero for start}
```

```bash
curl -v -H "Authorization: Bearer $TOKEN" -X GET "localhost:8080/api/s/post/2/comments?limit=2&offset=0"
```

```bash
Note: Unnecessary use of -X or --request, GET is already inferred.
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
> GET /api/s/post/2/comments?limit=2&offset=0 HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/8.0.1
> Accept: */*
> Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoxLCJmaXJzdF9uYW1lIjoiSmF5IiwibGFzdF9uYW1lIjoiU2luZ2giLCJhdmF0YXIiOiIiLCJqdGkiOiI1MmZiNDNlZC0xODk1LTQ4ZDAtOWFhOC1hNmNkYTU0MWI1YjciLCJleHAiOjE2ODQxMzg0OTJ9.A4ZJIALOHTdgEtUDiJoTwZLuriZkyQf4dobBNdG3mp4
> 
< HTTP/1.1 201 Created
< content-length: 153
< date: Mon, 08 May 2023 10:22:46 GMT
< 
* Connection #0 to host localhost left intact
[[{"id":1,"post_id":2,"user_id":1,"note":"Wow","created_at":"2023-05-08T10:09:34.083292"},{"id":1,"first_name":"Jay","last_name":"Singh","avatar":null}]]⏎
```

### Add Like

```
POST /api/s/post/{post_id}/like
```

```bash
curl -v -H "Authorization: Bearer $TOKEN" -X POST "localhost:8080/api/s/post/2/like"
```

```bash
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
> POST /api/s/post/2/like HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/8.0.1
> Accept: */*
> Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoxLCJmaXJzdF9uYW1lIjoiSmF5IiwibGFzdF9uYW1lIjoiU2luZ2giLCJhdmF0YXIiOiIiLCJqdGkiOiI1MmZiNDNlZC0xODk1LTQ4ZDAtOWFhOC1hNmNkYTU0MWI1YjciLCJleHAiOjE2ODQxMzg0OTJ9.A4ZJIALOHTdgEtUDiJoTwZLuriZkyQf4dobBNdG3mp4
> 
< HTTP/1.1 201 Created
< content-length: 27
< date: Mon, 08 May 2023 10:35:28 GMT
< 
* Connection #0 to host localhost left intact
{"message":"Post is liked"}⏎                
```

### Delete Like

```
DELETE /api/s/post/{post_id}/like
```

```bash
curl -v -H "Authorization: Bearer $TOKEN" -X DELETE "localhost:8080/api/s/post/2/like"
```

```bash
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
> DELETE /api/s/post/2/like HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/8.0.1
> Accept: */*
> Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoxLCJmaXJzdF9uYW1lIjoiSmF5IiwibGFzdF9uYW1lIjoiU2luZ2giLCJhdmF0YXIiOiIiLCJqdGkiOiI1MmZiNDNlZC0xODk1LTQ4ZDAtOWFhOC1hNmNkYTU0MWI1YjciLCJleHAiOjE2ODQxMzg0OTJ9.A4ZJIALOHTdgEtUDiJoTwZLuriZkyQf4dobBNdG3mp4
> 
< HTTP/1.1 201 Created
< content-length: 29
< date: Mon, 08 May 2023 10:37:39 GMT
< 
* Connection #0 to host localhost left intact
{"message":"Post is unliked"}⏎   
```

## Files

### Upload File

```
POST /api/s/file

Content-type: multipart/form-data

Payload: file
```

```bash
curl -v -H "Authorization: Bearer $TOKEN" -H "Content-Type: multipart/form-data"  -X POST "localhost:8080/api/s/file" -F "data=@Screenshot_20230505_174824.png"
```

```bash
Note: Unnecessary use of -X or --request, POST is already inferred.
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
> POST /api/s/file HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/8.0.1
> Accept: */*
> Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoxLCJmaXJzdF9uYW1lIjoiSmF5IiwibGFzdF9uYW1lIjoiU2luZ2giLCJhdmF0YXIiOiIiLCJqdGkiOiI1MmZiNDNlZC0xODk1LTQ4ZDAtOWFhOC1hNmNkYTU0MWI1YjciLCJleHAiOjE2ODQxMzg0OTJ9.A4ZJIALOHTdgEtUDiJoTwZLuriZkyQf4dobBNdG3mp4
> Content-Length: 292748
> Content-Type: multipart/form-data; boundary=------------------------37a397eb3866ad8d
> 
* We are completely uploaded and fine
< HTTP/1.1 200 OK
< content-length: 55
< date: Mon, 08 May 2023 11:16:34 GMT
< 
* Connection #0 to host localhost left intact
{"filename":"f9d8c2e6-a775-414f-9d68-8dbb006f1b5d.png"}⏎  
```

### Get file

```
GET /api/o/file/{filename}
```

```bash
curl -v -X GET "localhost:8080/api/o/file/f9d8c2e6-a775-414f-9d68-8dbb006f1b5d.png" --output a.png
```

```bash
Note: Unnecessary use of -X or --request, GET is already inferred.
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
> GET /api/o/file/f9d8c2e6-a775-414f-9d68-8dbb006f1b5d.png HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/8.0.1
> Accept: */*
> 
< HTTP/1.1 200 OK
< content-length: 292541
< content-disposition: inline; filename="f9d8c2e6-a775-414f-9d68-8dbb006f1b5d.png"
< last-modified: Mon, 08 May 2023 11:16:34 GMT
< etag: "b15803:476bd:6458da12:26713e1c"
< content-type: image/png
< accept-ranges: bytes
< date: Mon, 08 May 2023 11:20:48 GMT
< 
{ [65536 bytes data]
100  285k  100  285k    0     0   110M      0 --:--:-- --:--:-- --:--:--  139M
```
