#include <curl/curl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
struct string {
  char *ptr;
  size_t len;
};
struct curl_slist *headers = NULL;
void init_string(struct string *s) {
  s->len = 0;
  s->ptr = malloc(s->len + 1);
  if (s->ptr == NULL) {
    fprintf(stderr, "malloc() failed\n");
    exit(EXIT_FAILURE);
  }
  s->ptr[0] = '\0';
}

size_t writefunc(void *ptr, size_t size, size_t nmemb, struct string *s) {
  size_t new_len = s->len + size * nmemb;
  s->ptr = realloc(s->ptr, new_len + 1);
  if (s->ptr == NULL) {
    fprintf(stderr, "realloc() failed\n");
    exit(EXIT_FAILURE);
  }
  memcpy(s->ptr + s->len, ptr, size * nmemb);
  s->ptr[new_len] = '\0';
  s->len = new_len;

  return size * nmemb;
}
int wresponse(struct string s) {
  FILE *result = fopen("./res.json", "w");
  fprintf(result, "%s", s.ptr);
  free(s.ptr);
  return 1;
}
int setheader() {
  curl_slist_append(headers, "Accept: application/json");
  curl_slist_append(headers, "Content-Type: application/json");
  curl_slist_append(headers, "charset: utf-8");
  return 1;
}
int curlprocess(CURL *curl, char *folder) {
  CURLcode res;
  if (curl) {
    struct string s;
    init_string(&s);
    curl_easy_setopt(curl, CURLOPT_URL, "http://localhost:3001/getfile");
    curl_easy_setopt(curl, CURLOPT_POSTFIELDS, folder);
    curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, strlen(folder));
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, writefunc);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &s);
    res = curl_easy_perform(curl);

    if (res != CURLE_OK) {
      fprintf(stderr, "curl_easy_perform() failed: %s\n",
              curl_easy_strerror(res));
      return 0;
    } else {
      wresponse(s);
    }

    curl_easy_cleanup(curl);
  }
  return 1;
}
int main(void) {
  remove("res.json");
  char *folder = "folder=.";
  CURL *curl;
  curl = curl_easy_init();
  setheader();
  curlprocess(curl, folder);
  printf("\n");
  return 0;
}