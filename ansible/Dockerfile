FROM alpine/ansible:latest

COPY entry.sh /usr/local/bin/entry.sh
RUN chmod +x /usr/local/bin/entry.sh
ENTRYPOINT ["/usr/local/bin/entry.sh"]
