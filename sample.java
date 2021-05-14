/*
 * Long Comments
 * Also Work
 */

class Person {
    // Yesu yesu kawaii desu
    private int age; 
    private String name;

    public int getAge() {
        return age;
    }

    public String getName() {
        return name;
    }

    public Person(int age, String name) {
        this.age = age;
        this.name = name;
    }

    public void setAge(int age) {
        this.age = age;
    }

    public void setName(String name) {
        this.name = name;
    }

    public String is23() {
        if (age != 23) {
            return "Yesu";
        } else {
            return "";
        }
    }
}
