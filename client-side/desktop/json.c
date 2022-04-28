
#include <json-c/json.h>
#include <stdio.h>

int main() {

  json_object * jobj = json_object_new_object();

  json_object *jstring = json_object_new_string(".");

  json_object_object_add(jobj,"folder", jstring);

  printf ("The json object created: %s\n",json_object_to_json_string(jobj));

}
