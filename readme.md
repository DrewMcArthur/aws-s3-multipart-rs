# aws s3 multipart uploads CLI utility

[this tweet](https://twitter.com/smt_solvers/status/1673808407147495425)
mentioned that the aws cli was too heavyweight to justify its use
for simple s3 multipart uploads.

this repo contains rust code for a cli utility that mirrors the aws cli,

```
aws s3 cp local_file.txt s3://bucket/file.txt
```

note this is mostly copied over from these references:

- https://docs.aws.amazon.com/sdk-for-rust/latest/dg/rust_s3_code_examples.html
- https://github.com/awsdocs/aws-doc-sdk-examples/blob/main/rust_dev_preview/s3/src/bin/s3-multipart-upload.rs
