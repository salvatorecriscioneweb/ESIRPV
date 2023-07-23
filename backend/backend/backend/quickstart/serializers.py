from django.contrib.auth.models import User, Group
from .models import Client, ProductDetails, Order, ProductsList
from rest_framework import serializers

class UserSerializer(serializers.HyperlinkedModelSerializer):
    class Meta:
        model = User
        fields = ['url', 'username', 'email', 'groups']

class GroupSerializer(serializers.HyperlinkedModelSerializer):
    class Meta:
        model = Group
        fields = ['url', 'name']

class ClientSerializer(serializers.HyperlinkedModelSerializer):
    class Meta:
        model = Client
        fields = ['name', 'surname', 'client_id']

class ProductDetailsSerializer(serializers.HyperlinkedModelSerializer):
    class Meta:
        model = ProductDetails
        fields = ['name', 'price', 'image', 'sku']

class OrderSerializer(serializers.HyperlinkedModelSerializer):
    class Meta:
        model = Order
        fields = ['order_id', 'address', 'client', 'price']

class ProductListSerializer(serializers.HyperlinkedModelSerializer):
    class Meta:
        model = ProductsList
        fields = ['order', 'product', 'quantity']