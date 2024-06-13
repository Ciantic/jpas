# Bash script version to handle jpas.sqlite files

```bash
./jpas-init.sh # creates a jpas.sqlite if not created yet

# Set an entry:
echo '{ "secrets" : "foo", "name": "foo", "urls": ["https://example.com", "https://domain.fi"] }' | ./jpas-set.sh

# Get an entry by name or url
./jpas-get.sh --name foo | ./jpas-decrypt.sh
# or
./jpas-get.sh --url https://example.com | ./jpas-decrypt.sh

```