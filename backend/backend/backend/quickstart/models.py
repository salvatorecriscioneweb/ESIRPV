from django.db import models

# Create your models here.


class Client(models.Model):
    name = models.CharField(max_length=40)
    surname = models.CharField(max_length=40)
    id = models.CharField(max_length=10, default="", primary_key=True)
    address = models.CharField(max_length=500, default='')

    def __str__(self):
        return self.name + ' ' + self.surname


class ProductDetails(models.Model):
    name = models.CharField(max_length=100)
    price = models.FloatField()
    image = models.ImageField(upload_to='image')
    sku = models.CharField(max_length=5)  # Limit to 5 letter the sku

    def __str__(self):
        return self.name


class Order(models.Model):
    order_id = models.CharField(max_length=100, default="")
    address = models.CharField(max_length=500)
    client = models.ForeignKey(Client, on_delete=models.CASCADE)
    timestamp = models.CharField(max_length=100, default="")

    def __str__(self) -> str:
        return self.order_id


class ProductsList(models.Model):
    order = models.ForeignKey(Order, on_delete=models.CASCADE)
    number = models.CharField(max_length=10, default='')
    product = models.ForeignKey(ProductDetails, on_delete=models.CASCADE)
    quantity = models.IntegerField(default=1)

    def __str__(self) -> str:
        return (self.order)


class Documents(models.Model):
    id = models.CharField(max_length=30, primary_key=True, default="")
    file = models.CharField(max_length=500, default='')
    hash = models.CharField(max_length=60, default='')
    timestamp = models.CharField(max_length=100, default='')
