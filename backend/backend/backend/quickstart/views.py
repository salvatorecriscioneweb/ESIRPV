from os import getcwd
import subprocess
from django.http import FileResponse
from django.shortcuts import render
from django.contrib.auth.models import User, Group
from .models import Client, ProductDetails, Order, ProductsList
from rest_framework import viewsets, permissions, status
from rest_framework.decorators import api_view
from rest_framework.response import Response
from .serializers import UserSerializer, GroupSerializer, ClientSerializer, ProductDetailsSerializer, ProductListSerializer, OrderSerializer


class UserViewSet(viewsets.ModelViewSet):
    queryset = User.objects.all().order_by('-date_joined')
    serializer_class = UserSerializer
    permission_classes = [permissions.IsAuthenticated]


class GroupViewSet(viewsets.ModelViewSet):
    queryset = Group.objects.all()
    serializer_class = GroupSerializer
    permission_classes = [permissions.IsAuthenticated]


class ClientViewSet(viewsets.ModelViewSet):
    queryset = Client.objects.all()
    serializer_class = ClientSerializer
    permission_classes = [permissions.IsAuthenticated]


class ProductDetailsViewSet(viewsets.ModelViewSet):
    queryset = ProductDetails.objects.all()
    serializer_class = ProductDetailsSerializer
    permission_classes = []


class OrderViewSet(viewsets.ModelViewSet):
    queryset = Order.objects.all()
    serializer_class = OrderSerializer
    permission_classes = []


class ProductsListViewSet(viewsets.ModelViewSet):
    queryset = ProductsList.objects.all()
    serializer_class = ProductListSerializer
    permission_classes = [permissions.IsAuthenticated]

# API Reply


DB_FILE = '../db.sqlite3'


@api_view(['GET'])
def getVersionBin(request):
    res = dict()
    res['version'] = "0.0.1"
    res['version'] = process = subprocess.run(
        [getcwd() + "/backend/quickstart/utils/rust-generate-flush-db", "version", getcwd() + "/test.sqlite3"], capture_output=True).stdout

    return Response(res)


@api_view(['GET'])
def generateDocument(request):
    res = dict()
    orderId = request.GET.get('orderId')
    orderQuery = Order.objects.filter(order_id=orderId)

    if (orderQuery.count() == 0):
        return Response('ORDER_NOT_FOUND', status=404)

    order = orderQuery.values().first()

    if (not order):
        return Response('UNEXPECTED ERROR', status=400)

    # fix security issue
    process = subprocess.run(
        [getcwd() + "/backend/quickstart/utils/rust-generate-flush-db", "document", order['address'].upper(), order['order_id'], getcwd() + "/test.sqlite3"], capture_output=True)

    if len(process.stderr.splitlines()) == 0:
        res['output'] = process.stdout.splitlines()

        response = FileResponse(
            open(getcwd() + '/test_data/' + order['order_id'] + '.pdf', 'rb'), content_type='application/pdf')

        response['Content-Disposition'] = 'attachment; filename="' + \
            order['order_id'] + '.pdf"'
        return response
    else:
        return Response(res)


@api_view(['GET'])
def getOrders(request):
    orders = Order.objects.all()

    def map_func(item):
        productsLists = ProductsList.objects.all().filter(order_id=item['id'])
        clientDb = Client.objects.all().filter(id=item['client_id'])
        return {
            "order": item,
            "list": productsLists.values(),
            "client": clientDb.values()
        }

    return Response({
        "orders": map(map_func, orders.values()),
    })
