#!/usr/bin/env python3

import argparse
import requests

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('route', help="Route to store the doc under")
    parser.add_argument('--url', '-u', default='http://localhost:8000', help="Server URL")
    parser.add_argument('--template', '-t', default='page')
    parser.add_argument('--content', '-c', required=True)
    args = parser.parse_args()

    with open(args.content) as f:
        content = f.read()
    resp = requests.post(args.url, json={
        'route': args.route,
        'template': args.template,
        'content': content
    })
    print(resp.status_code)
    print(resp.text)
