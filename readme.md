# aws s3 multipart uploads CLI utility

[this tweet](https://twitter.com/smt_solvers/status/1673808407147495425)
mentioned that the aws cli was too heavyweight to justify its use
for simple s3 multipart uploads.

this repo contains rust code for a cli utility that mirrors the aws cli,

```
aws s3 cp local_file.txt s3://bucket/file.txt
```
