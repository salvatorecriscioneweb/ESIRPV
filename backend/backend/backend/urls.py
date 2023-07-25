from django.urls import include, path
from rest_framework import routers
from backend.quickstart import views
from rest_framework.authtoken import views as auth_views

router = routers.DefaultRouter()
router.register(r'users', views.UserViewSet)
router.register(r'groups', views.GroupViewSet)
router.register(r'clients', views.ClientViewSet)
router.register(r'product-details', views.ProductDetailsViewSet)
router.register(r'orders', views.OrderViewSet)
router.register(r'product-lists', views.ProductsListViewSet)
# Wire up our API using automatic URL routing.
# Additionally, we include login URLs for the browsable API.
urlpatterns = [
    path('', include(router.urls)),
    path('api-auth/', include('rest_framework.urls', namespace='rest_framework')),
    path('api-token-auth/', auth_views.obtain_auth_token)
]