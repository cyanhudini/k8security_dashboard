import os
import sys
import psycopg2
import smtplib
from email.message import EmailMessage


DB_NAME = os.getenv("POSTGRESQL_DB_NAME")
DB_USER = os.getenv("POSTGRESQL_USERNAME")
DB_PASS = os.getenv("POSTGRESQL_PASSWORD")
DB_HOST = os.getenv("HOST")
DB_PORT = os.getenv("PORT")


SMTP_SERVER = os.getenv("SMTP_SERVER_NAME") 

SENDER_EMAIL = os.getenv("SENDER_EMAIL")
EMAIL_USER = os.getenv("EMAIL_USERNAME") 
EMAIL_PASS = os.getenv("EMAIL_PASSWORD")


def get_db_connection():
  
    try:
        conn = psycopg2.connect(
            dbname=DB_NAME,
            user=DB_USER,
            password=DB_PASS,
            host=DB_HOST,
            port=5432,
            connect_timeout=15  
        )
        
        return conn
    except psycopg2.OperationalError as e:
        print(f"Database connection failed: {e}", file=sys.stderr)
        return None


def get_active_emails(conn):
    
    with conn.cursor() as cur:
    
        cur.execute("SELECT email_adress FROM emails WHERE receiving = true")
        emails = [row[0] for row in cur.fetchall()]

        return emails


def get_vulnerability_data(conn):

    
    with conn.cursor() as cur:
        cur.execute("SELECT * FROM vulnerability;")
        if cur.rowcount == 0:
            
            return None, None
            
        headers = [desc[0] for desc in cur.description]
        rows = cur.fetchall()
        
        return headers, rows


def format_data_for_email(headers, rows):
    
    if not rows:
        return "No new vulnerabilities to report."
    
   
    header_line = " | ".join(str(header) for header in headers)
    
    
    data_lines = []
    for row in rows:
        data_lines.append(" | ".join(str(item) for item in row))
        
    
    return header_line + "\n" + "-" * len(header_line) + "\n" + "\n".join(data_lines)


def send_email(recipients, subject, body):

    if not recipients:
        
        return


        
    server = smtplib.SMTP_SSL(SMTP_SERVER, 465)
    server.login(EMAIL_USER, EMAIL_PASS)
    
    
    for email in recipients:
    
        subject = "High Severity Vulnerabilities im Kubernetes CLuster"

        msg = EmailMessage()
        msg['From'] = SENDER_EMAIL
        msg['To'] = email
        msg['Subject'] = subject
        msg.set_content(body)
        
        
        server.sendmail(SENDER_EMAIL, email, msg.as_string())
    server.quit()


def main():
 
    conn = get_db_connection()

    if not conn:
        sys.exit(1)

    try:

        recipient_list = get_active_emails(conn)
        vuln_headers, vuln_rows = get_vulnerability_data(conn)

        if recipient_list and vuln_rows:
            email_body = format_data_for_email(vuln_headers, vuln_rows)
            email_subject = "Vulnerability Report for Kubernetes Cluster"
            
           
            send_email(recipient_list, email_subject, email_body)
           

    finally:
    
        if conn:
            conn.close()

if __name__ == "__main__":

    main()
