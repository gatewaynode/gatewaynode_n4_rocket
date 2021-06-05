#!/usr/bin/env bash
# set -euo pipefail

echo "---";
echo "Generate local site mirror";
echo "---";
if [ -d gatewaynode.com ]; then
	wget --mirror --convert-links --adjust-extension --page-requisites --no-parent http://locaLhost:8000;
	wget http://localhost:8000/sitemap.xml;
	mv -v sitemap.xml localhost\:8000/sitemap.xml;
	wget http://localhost:8000/static/css/comments.css;
	mv -v comments.css localhost\:8000/static/css/comments.css;
else
	echo "gatewaynode.com dir not found, exiting";
	exit;
fi
echo "---";
echo "Move site to distribution folder";
echo "---";
if [ -d 'localhost:8000' ]; then
	rm -rvf gatewaynode.com/*
	mv -vf localhost\:8000/* gatewaynode.com/;
	rm -rvf localhost:8000;
else
	echo "Mirroring failed";
	exit;
fi

echo "---";
echo "Sync to S3 bucket and invalidate CDN";
echo "---";
if [ -f ~/.aws/credentials ]; then
	aws s3 sync gatewaynode.com/. s3://gatewaynode;
	aws cloudfront create-invalidation --distribution-id E221I4FAFJZ96X --paths "/*";
else
	echo "AWS credentials not found, sync and invalidate aborted";
	exit;
fi
