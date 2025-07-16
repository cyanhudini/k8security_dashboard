import smtplib
import psycopg2
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart
from dotenv import load_dotenv
import os

load_dotenv()

db_name = os.getenv("POSTGRESQL_DB_NAME")
db_user = os.getenv("POSTGRESQL_USERNAME")
db_pass = os.getenv("POSTGRESQL_PASSWORD")
db_host = os.getenv("HOST")
db_port = os.getenv("PORT")

sender_email = os.getenv("SENDER_EMAIL")
email_user = os.getenv("EMAIL_USERNAME")
email_pass = os.getenv("EMAIL_PASSWORD")

def send_email(report, receiver_emails):
    server = smtplib.SMTP_SSL('')
    server.login(email_user, email_pass)
    for email in receiver_emails:
    
        subject = "High Severity Vulnerabilities im Kubernetes CLuster"

        msg = MIMEMultipart()
        msg['From'] = sender_email
        msg['To'] = ", ".join(email)
        msg['Subject'] = subject

        body = "Im Anhang findest du eine Zusammenfassung der durch Trivy gefunden Vulnerabilities mit einer Schwere höher HIGH."
        msg.attach(MIMEText(body, 'plain'))

        attachment = MIMEText(report, 'plain')
        ## muss so geändert werden das nur neuen Vulns geschickt werden oder nur solche mit Severity HIGH oder höher
        attachment.add_header('Content-Disposition', 'attachment', filename="report.json")
        msg.attach(attachment)
        
        
        server.sendmail(sender_email, email, msg.as_string())
    server.quit()

def get_active_emails():
    conn = psycopg2.connect(
        dbname=db_name,
        user=db_user,
        password=db_pass,
        host=db_host,
        port=db_port
)
    cur = conn.cursor()
    cur.execute("SELECT email_adress FROM emails WHERE receiving = true")
    emails = [row[0] for row in cur.fetchall()]
    cur.close()
    conn.close()
    return emails

def main():
    with open("report.json", "r") as f:
        report_content = f.read()
    if report_content:
        receivers = get_active_emails()
        if receivers:
            send_email(report_content, receivers)
        else:
            print("No active emails found.")
    else:
        
        print("Keine zu sendende Datei gefunden")

if __name__ == "__main__":
    main()
