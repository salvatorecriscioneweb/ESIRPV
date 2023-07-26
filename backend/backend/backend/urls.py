from django.urls import include, path
from django.contrib import admin
from rest_framework import routers
from backend.quickstart import views
from rest_framework.authtoken import views as auth_views
from django.conf.urls.static import static
from django.conf import settings

# Wire up our API using automatic URL routing.
# Additionally, we include login URLs for the browsable API.
urlpatterns = [
    path('admin/', admin.site.urls),
    path('version', views.getVersionBin),
    path('document', views.generateDocument),
    path('get-orders', views.getOrders),
    path('api-auth/', include('rest_framework.urls', namespace='rest_framework')),
    path('api-token-auth/', auth_views.obtain_auth_token)
] + static(settings.MEDIA_URL, document_root=settings.MEDIA_ROOT)
