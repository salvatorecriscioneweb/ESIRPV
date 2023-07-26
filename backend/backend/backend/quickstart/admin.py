from django.contrib import admin
from .models import Client, ProductDetails, Order, ProductsList, Documents

# Register your models here.
admin.site.register(Client)
admin.site.register(ProductDetails)
admin.site.register(Order)
admin.site.register(ProductsList)
admin.site.register(Documents)
