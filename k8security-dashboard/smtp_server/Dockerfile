FROM python:3.9-slim

RUN apt-get -y update && apt-get install -y \
    libpq-dev \
    cron \
    curl \
    python3-pip

RUN curl -sfL https://raw.githubusercontent.com/aquasecurity/trivy/main/contrib/install.sh | sh -s -- -b /usr/local/bin v0.60.0


COPY entrypoint.sh /home/entrypoint.sh

COPY cronjob /etc/cron.d/cron
RUN chmod 0644 /etc/cron.d/cron
RUN chmod +x /home/entrypoint.sh

COPY email_sender.py ./

COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt

RUN touch /var/log/cron.log

ENTRYPOINT ["/bin/sh", "/home/entrypoint.sh"]

EXPOSE 1234

CMD ["cron", "-f"]