#!/usr/bin/env python3
from http.server import HTTPServer, BaseHTTPRequestHandler
import json
import urllib.parse

class TestHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header('Content-type', 'text/plain')
        self.end_headers()
        response = f"�� 欢迎使用测试服务器!\n\n请求路径: {self.path}\n方法: GET"
        self.wfile.write(response.encode())
    
    def do_POST(self):
        content_length = int(self.headers.get('Content-Length', 0))
        body = self.rfile.read(content_length).decode('utf-8')
        
        self.send_response(200)
        self.send_header('Content-type', 'application/json')
        self.end_headers()
        
        response = {
            "message": "POST 请求已接收",
            "path": self.path,
            "headers": dict(self.headers),
            "body": body,
            "content_type": self.headers.get('Content-Type', 'unknown')
        }
        
        self.wfile.write(json.dumps(response, indent=2, ensure_ascii=False).encode())

if __name__ == "__main__":
    server = HTTPServer(('localhost', 33000), TestHandler)
    print("🚀 启动测试服务器在 http://localhost:33000")
    print("按 Ctrl+C 停止服务器")
    server.serve_forever()