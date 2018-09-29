#!/usr/bin/env python3

import argparse
import requests

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('route')
    parser.add_argument('--template', '-t', default='page')
    parser.add_argument('--content', '-c', default='')
    args = parser.parse_args()
    argd = vars(args)

    resp = requests.post(f'http://localhost:8000/', json=argd)
    print(resp.status_code)
    print(resp.text)
