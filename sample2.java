/*
 * Long Comments
 * Also Work
 */

class Person {
    // Yesu yesu kawaii desu
    private int age; 
    private String name;
    private Person[] family = { new Person(23, "Father"), new Person(23, "Mother") };

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
            return "Nested blocks in functions also work";
        }
    }

    public static void main(String[] args) {
        System.out.println("Khello World");
    }

    protected void protectedMethod() {
        System.out.println("" + age);
    }
}
