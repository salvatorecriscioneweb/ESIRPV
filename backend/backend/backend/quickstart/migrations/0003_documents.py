# Generated by Django 4.2.3 on 2023-07-24 19:47

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('quickstart', '0002_remove_client_client_id_remove_order_price_and_more'),
    ]

    operations = [
        migrations.CreateModel(
            name='Documents',
            fields=[
                ('id', models.CharField(default='', max_length=30, primary_key=True, serialize=False)),
                ('file', models.CharField(default='', max_length=500)),
                ('hash', models.CharField(default='', max_length=60)),
                ('timestamp', models.CharField(default='', max_length=100)),
            ],
        ),
    ]